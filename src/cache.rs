use reqwest::Client;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::time::SystemTime;
use utils::span_of;

const DEFAULT_TARGET_LANGUAGE: &'static str = "ja";
const CACHE_FILE: &'static str = ".trs-cache";
const HIGH_WATER_MARK: u64 = 16 * 1000; // 16kib
const NULL: u8 = 0;
const CONFIG_ENDPOINT: &'static str = env!("GCP_FN_ENDPOINT");

pub enum Namespace {
  Translate,
  Dictionary,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ApiKeys {
  pub gcloud_translate_api_key: String,
  pub oxford_api_id: String,
  pub oxford_api_key: String,
}

impl ApiKeys {
  pub fn new() -> Self {
    let http_client = Client::new().expect("Create HTTP client is failed");
    let mut buffer = String::new();
    http_client
      .get(CONFIG_ENDPOINT)
      .send()
      .expect("send Request failed")
      .read_to_string(&mut buffer)
      .expect("read response failed");
    serde_json::from_str::<ApiKeys>(&buffer).expect("Cloud function does not return value propery")
  }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct FsCacheValue(SystemTime, String);
impl Display for FsCacheValue {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", self.1)
  }
}

#[derive(Deserialize, Serialize)]
pub struct FSCache {
  version: u8,
  language: String,
  pub api_keys: ApiKeys,
  translate: HashMap<String, HashMap<String, FsCacheValue>>,
  dictionary: HashMap<String, FsCacheValue>,
}

impl FSCache {
  #[cfg(not(debug_assertions))]
  fn get_cache() -> PathBuf {
    env::home_dir()
      .and_then(|p| Some(p.join(CACHE_FILE)))
      .unwrap()
  }

  #[cfg(debug_assertions)]
  fn get_cache() -> PathBuf {
    env::current_dir()
      .and_then(|p| Ok(p.join(CACHE_FILE)))
      .unwrap()
  }

  pub fn new() -> Self {
    let cache_file = Self::get_cache();
    let maybe_json = match fs::File::open(&cache_file) {
      Ok(mut file) => {
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf);
        let json = span_of("decompress", || FSCache::decompress(buf));

        match serde_json::from_str::<FSCache>(&json) {
          Ok(json) => Some(json),
          Err(_) => None,
        }
      }
      Err(_) => None,
    };
    match maybe_json {
      Some(json) => json,
      None => {
        let _ = fs::File::create(cache_file);
        let mut fs_cache = FSCache {
          version: 3,
          language: DEFAULT_TARGET_LANGUAGE.to_owned(),
          translate: HashMap::new(),
          dictionary: HashMap::new(),
          api_keys: ApiKeys::new(),
        };
        fs_cache.update_cache();
        fs_cache
      }
    }
  }

  pub fn get(&self, namespace: &Namespace, key: &String) -> Option<String> {
    use self::Namespace::*;
    match namespace {
      &Dictionary => self.dictionary.get(key).and_then(|v| Some(v.1.to_owned())),
      &Translate => self
        .translate
        .get(&self.language)
        .and_then(|map| map.get(key))
        .and_then(|v| Some(v.1.to_owned())),
    }
  }

  pub fn get_language(&self) -> String {
    self.language.clone()
  }

  pub fn get_all(&self) -> String {
    let language = &self.language;
    let translate = &self
      .translate
      .iter()
      .map(|(ref lan, ref pairs)| {
        println!("{:?}", pairs);
        format!(
          r"Language: {}
{}",
          lan,
          pairs
            .iter()
            .map(|(ref key, ref value)| format!("{}: {}", key, value))
            .collect::<Vec<_>>()
            .join("\n")
        )
      })
      .collect::<Vec<_>>()
      .join("\n");
    let dictionary = &self
      .dictionary
      .iter()
      .map(|(ref key, ref value)| format!("{}:\n{}", key, value))
      .collect::<Vec<_>>()
      .join("\n");

    format!(
      r"Target language: {}

Definitions:
  {}

Translates:
  {}",
      language, dictionary, translate
    )
  }

  pub fn set_language(&mut self, language: &String) {
    self.language = language.to_owned();
    self.update_cache();
  }

  pub fn set(&mut self, namespace: &Namespace, key: &String, value: &String) {
    use self::Namespace::*;
    match namespace {
      &Dictionary => {
        self.dictionary.insert(
          key.to_owned(),
          FsCacheValue(SystemTime::now(), value.to_owned()),
        );
      }
      &Translate => {
        let translation_map = match self.translate.get_mut(&self.language) {
          Some(mut map) => {
            map.insert(
              key.to_owned(),
              FsCacheValue(SystemTime::now(), value.to_owned()),
            );
            None
          }
          None => {
            let mut map = HashMap::new();
            map.insert(
              key.to_owned(),
              FsCacheValue(SystemTime::now(), value.to_owned()),
            );
            Some(map)
          }
        };

        match translation_map {
          Some(map) => {
            self.translate.insert(self.language.to_owned(), map);
          }
          None => {}
        };
      }
    };
    self.update_cache();
  }

  fn update_cache(&mut self) {
    match (
      fs::File::create(&Self::get_cache()),
      serde_json::to_string_pretty(&self),
    ) {
      (Ok(mut file), Ok(buf)) => {
        let _ = span_of("compress", || file.write_all(&mut FSCache::compress(&buf)));

        #[cfg(debug_assertions)]
        let _ = if let Ok(mut file) = fs::File::create("fixture/.trs-cache-raw") {
          let mut json = buf.clone();
          let _ = file.write_all(&mut json.as_bytes());
        } else {
        };
      }
      (Err(e), _) => unreachable!(
        "Something wrong, cache file did not initialize correctly\n{:?}",
        e
      ),
      (_, Err(e)) => unreachable!("Can not parse cache data correctly\n{:?}", e),
    };
  }

  fn compress(raw: &String) -> Vec<u8> {
    let (compressed, table) = compress(raw);
    let mut table_serialized = serialize_table(table.clone());
    let (mut body_serialized, pad_of_last) = bit_of_string(compressed);
    table_serialized.push(NULL);
    table_serialized.push(pad_of_last);
    table_serialized.push(NULL);
    table_serialized.append(&mut body_serialized);
    table_serialized
  }

  fn decompress(raw: Vec<u8>) -> String {
    let mut raws = raw.splitn(3, |code| code == &NULL);
    let table_raw = raws
      .next()
      .expect(format!("{}:{}", file!(), line!()).as_ref());

    let pad_of_last = match raws
      .next()
      .expect(format!("{}:{}", file!(), line!()).as_ref())
      .first()
    {
      Some(p) => *p,
      None => 0,
    };

    let cache_raw = raws
      .next()
      .expect(format!("{}:{}", file!(), line!()).as_ref());
    let cache_raw = match pad_of_last {
      0 => cache_raw.split_at(1).1,
      _ => cache_raw,
    };
    assert!(raws.next().is_none());

    let table = deserialize_table(table_raw.to_vec());
    let cache_string = string_of_bit((cache_raw.to_vec(), pad_of_last.clone()));
    let cache = span_of("decompress-inner", || decompress(cache_string, &table));
    cache
  }

  pub fn garbage_colloect(&mut self) -> io::Result<()> {
    let cache_file = Self::get_cache();
    let file = fs::File::open(&cache_file)?;
    let file_size = file.metadata().and_then(|meta| Ok(meta.len()))?;
    if file_size < HIGH_WATER_MARK {
      return Ok(());
    }

    fn collect_young_cache(map: HashMap<String, FsCacheValue>) -> HashMap<String, FsCacheValue> {
      let delete_range = 3;
      let mut sorted = map.into_iter().collect::<Vec<_>>();
      sorted.sort_by(|&(_, FsCacheValue(a, _)), &(_, FsCacheValue(b, _))| a.cmp(&b));

      let young_caches = if sorted.len() >= delete_range {
        sorted.split_off(delete_range)
      } else {
        sorted
      };
      young_caches.into_iter().collect()
    }

    let young_dictionary = collect_young_cache(self.dictionary.to_owned());
    let young_translates = self
      .translate
      .to_owned()
      .into_iter()
      .map(|(lang, map)| {
        let young = collect_young_cache(map.to_owned());
        (lang, young)
      })
      .collect::<HashMap<String, HashMap<String, FsCacheValue>>>();
    self.dictionary = young_dictionary;
    self.translate = young_translates;
    self.update_cache();
    self.garbage_colloect()
  }
}

// 'a': "100", 5
type HuffmanTable = HashMap<char, (String, u16)>;
type HuffmanTableInvert = HashMap<String, char>;
type HuffmanTableSerializable = HashMap<String, u16>;

fn serialize_table(table: HuffmanTable) -> Vec<u8> {
  let table = table
    .into_iter()
    .map(|(k, v)| (format!("{}", k), v.1))
    .collect::<HuffmanTableSerializable>();
  match serde_json::to_vec(&table) {
    Ok(x) => x,
    Err(e) => unreachable!("{}:{} {:?}", file!(), line!(), e),
  }
}

fn deserialize_table(from: Vec<u8>) -> HuffmanTable {
  let serializable = match serde_json::from_slice::<HuffmanTableSerializable>(&from) {
    Ok(x) => x,
    Err(e) => unreachable!("{}:{} {:?}", file!(), line!(), e),
  };

  let mut leafs = serializable
    .into_iter()
    .map(|(character, size)| {
      let codes = character.as_bytes();
      let mut buf = [0u8; 4];
      for i in 0..4 {
        if let Some(code) = codes.get(i) {
          buf[i] = *code;
        };
      }
      HuffmanTree::Leaf((buf, size))
    })
    .collect::<Vec<HuffmanTree>>();

  leafs.sort_by(|a, b| match (a, b) {
    (&HuffmanTree::Leaf(a), &HuffmanTree::Leaf(b)) => a.0.cmp(&b.0),
    _ => unreachable!(),
  });
  HuffmanTree::build_tree(leafs).get_table()
}

#[derive(Debug, Clone, PartialEq)]
enum HuffmanTree {
  Leaf(([u8; 4], u16)),
  Node {
    zero: Box<HuffmanTree>,
    one: Box<HuffmanTree>,
    probability: u16,
  },
}

impl HuffmanTree {
  fn get_codes(&self, code: &String, mut code_table: &mut HuffmanTable) {
    use self::HuffmanTree::*;

    match self {
      &Leaf((codes, size)) => {
        let c = String::from_utf8(codes.to_vec())
          .expect(format!("{}:{} {:#?}", file!(), line!(), codes).as_str())
          .remove(0);
        code_table.insert(c, (code.to_owned(), size));
      }
      &Node {
        ref zero, ref one, ..
      } => {
        zero.get_codes(&format!("{}0", code), &mut code_table);
        one.get_codes(&format!("{}1", code), &mut code_table);
      }
    }
  }

  fn get_table(&self) -> HuffmanTable {
    use self::HuffmanTree::*;

    let mut code_table = HashMap::new();
    match self {
      &Leaf((codes, size)) => {
        let c = String::from_utf8(codes.to_vec())
          .expect(format!("{}:{} {:#?}", file!(), line!(), codes).as_str())
          .remove(0);
        code_table.insert(c, ("0".to_owned(), size));
        code_table
      }
      &Node {
        ref zero, ref one, ..
      } => {
        zero.get_codes(&"0".to_owned(), &mut code_table);
        one.get_codes(&"1".to_owned(), &mut code_table);
        code_table
      }
    }
  }

  fn get_probability(&self) -> u16 {
    use self::HuffmanTree::*;
    match self {
      &Leaf((_, p)) => p,
      &Node { probability, .. } => probability,
    }
  }

  fn new(source: &String) -> Self {
    let mut leafs: Vec<HuffmanTree> = Self::count(source)
      .iter()
      .map(|&(_, c)| HuffmanTree::Leaf(c))
      .collect();

    leafs.sort_by(|a, b| match (a, b) {
      (&HuffmanTree::Leaf(a), &HuffmanTree::Leaf(b)) => a.0.cmp(&b.0),
      _ => unreachable!(),
    });

    HuffmanTree::build_tree(leafs)
  }

  fn build_tree(mut trees: Vec<Self>) -> Self {
    match trees.len() {
      0 => unreachable!("Could not found trees, something become wrong."),
      1 => trees.first().unwrap().clone(),
      _ => {
        trees.sort_by(|a, b| a.get_probability().cmp(&b.get_probability()));
        let mut remains = trees.split_off(2);
        let small_fst = trees.get(0).unwrap();
        let small_snd = trees.get(1).unwrap();
        let new_tree = HuffmanTree::Node {
          zero: Box::new(small_snd.clone()),
          one: Box::new(small_fst.clone()),
          probability: small_fst.get_probability() + small_snd.get_probability(),
        };
        remains.push(new_tree);
        HuffmanTree::build_tree(remains)
      }
    }
  }

  fn count(source: &String) -> Vec<(char, ([u8; 4], u16))> {
    let mut length_of_chars = source
      .chars()
      .fold(HashMap::new(), |mut acc: HashMap<char, (char, u16)>, c| {
        if let Some(&(_, n)) = acc.get(&c) {
          acc.insert(c, (c, n + 1));
        } else {
          acc.insert(c, (c, 1));
        };
        acc
      })
      .into_iter()
      .map(|(_, (c, n))| {
        let mut buf = [0; 4];
        c.encode_utf8(&mut buf);
        (c, (buf, n))
      })
      .collect::<Vec<(char, ([u8; 4], u16))>>();
    length_of_chars.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    length_of_chars
  }
}

fn compress(source: &String) -> (String, HuffmanTable) {
  let table = HuffmanTree::new(source).get_table();
  (
    source
      .chars()
      .enumerate()
      .fold("".to_owned(), |acc, (_, c)| {
        let code = table.get(&c).expect(
          format!(
            "{}:{} A character [{:?}] did not code correctly to the Haffman-table",
            file!(),
            line!(),
            c
          ).as_str(),
        );
        format!("{}{}", acc, code.0)
      }),
    table,
  )
}

fn decompress(mut source: String, table: &HuffmanTable) -> String {
  let invert_table = table
    .into_iter()
    .map(|(k, v)| (v.clone().0, k.clone()))
    .collect::<HuffmanTableInvert>();

  let mut result_buf = String::new();
  let mut code_buf = String::new();

  // FIXME: Consider performance problem caused here.
  span_of("iterate_over_source", || {
    while (source.len() > 0) || (code_buf.len() > 0) {
      match invert_table.get(&code_buf) {
        Some(next_char) => {
          code_buf.clear();
          result_buf = format!("{}{}", result_buf, next_char);
        }
        None => {
          let source_tmp = source.to_owned();
          let (c, next) = source_tmp.split_at(1);
          code_buf = format!("{}{}", code_buf, c);
          source = next.to_owned();
        }
      };
    }
  });
  result_buf
}

fn bit_of_string(mut from: String) -> (Vec<u8>, u8) {
  let mut buf = Vec::new();
  let mut pad_of_last = 0;
  while from.len() > 0 {
    let from_tmp = from.clone();
    let next = if from_tmp.len() < 8 {
      pad_of_last = 8 - from_tmp.len() as u8;
      from = "".to_owned();
      from_tmp.as_str()
    } else {
      let (next, rest) = from_tmp.split_at(8);
      from = rest.to_owned();
      next
    };

    let result = u8::from_str_radix(next, 2)
      .expect(format!("{}:{} Can not parse correctly [{}]", file!(), line!(), next).as_ref());
    buf.push(result);
  }
  (buf, pad_of_last)
}

fn string_of_bit(from: (Vec<u8>, u8)) -> String {
  let (from, pad_of_last) = from;
  let last_index = from.len() - 1;
  from
    .into_iter()
    .enumerate()
    .map(|(idx, n)| {
      let mut bit_string = format!("{:b}", n);
      if bit_string.len() < 8 {
        if idx == last_index {
          while (bit_string.len() as u8) < (8 - pad_of_last) {
            bit_string = format!("0{}", bit_string);
          }
          bit_string
        } else {
          while (bit_string.len() as u8) < 8 {
            bit_string = format!("0{}", bit_string);
          }
          bit_string
        }
      } else {
        bit_string
      }
    })
    .collect::<String>()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_haffman_count() {
    let x = "AAAAABBBBCCCDDE".to_owned();
    assert_eq!(
      vec![
        ('E', ([69, 0, 0, 0], 1)),
        ('D', ([68, 0, 0, 0], 2)),
        ('C', ([67, 0, 0, 0], 3)),
        ('B', ([66, 0, 0, 0], 4)),
        ('A', ([65, 0, 0, 0], 5)),
      ],
      HuffmanTree::count(&x.to_owned())
    );
  }

  #[test]
  fn test_haffman_simple_tree() {
    use self::HuffmanTree::*;
    let x = HuffmanTree::new(&"AAB".to_owned());
    assert_eq!(
      Node {
        zero: Box::new(Leaf(([65, 0, 0, 0], 2))),
        one: Box::new(Leaf(([66, 0, 0, 0], 1))),
        probability: 3,
      },
      x
    );
  }

  #[test]
  fn test_haffman_bit_complecated_tree() {
    use self::HuffmanTree::*;
    let x = HuffmanTree::new(&"AAABBC".to_owned());
    assert_eq!(
      Node {
        zero: Box::new(Node {
          zero: Box::new(Leaf(([66, 0, 0, 0], 2))),
          one: Box::new(Leaf(([67, 0, 0, 0], 1))),
          probability: 3,
        }),
        one: Box::new(Leaf(([65, 0, 0, 0], 3))),
        probability: 6,
      },
      x
    );
  }

  #[test]
  fn test_haffman_tree() {
    use self::HuffmanTree::*;
    let x = HuffmanTree::new(&"AAAAABBBBCCCDDE".to_owned());
    assert_eq!(
      Node {
        zero: Box::new(Node {
          zero: Box::new(Leaf(([65, 0, 0, 0], 5))),
          one: Box::new(Leaf(([66, 0, 0, 0], 4))),
          probability: 9,
        }),
        one: Box::new(Node {
          zero: Box::new(Node {
            zero: Box::new(Leaf(([68, 0, 0, 0], 2))),
            one: Box::new(Leaf(([69, 0, 0, 0], 1))),
            probability: 3,
          }),
          one: Box::new(Leaf(([67, 0, 0, 0], 3))),
          probability: 6,
        }),
        probability: 15,
      },
      x
    );
  }

  #[test]
  fn test_haffman_table() {
    let x = HuffmanTree::new(&"AAAAABBBBCCCDDE".to_owned()).get_table();
    /*
    1000010
    1000011
    1000101
    1000001
    1000100
     */
    assert_eq!(
      vec![
        ('A', ("00".to_owned(), 5)),
        ('B', ("01".to_owned(), 4)),
        ('C', ("11".to_owned(), 3)),
        ('D', ("100".to_owned(), 2)),
        ('E', ("101".to_owned(), 1)),
      ].into_iter()
        .collect::<HuffmanTable>(),
      x
    );
  }

  #[test]
  fn test_compress_effectively() {
    let expect = "AAAAABBBBCCCDDE".to_owned();
    let compressed = compress(&expect);
    let not_compressed = format!(
      "{:?}",
      expect
        .as_bytes()
        .iter()
        .fold("".to_owned(), |acc, b| format!("{}{:b}", acc, b))
    );
    assert!((compressed.0.len() as f32 / not_compressed.len() as f32) < 0.5);
    assert!(compressed.0.len() < not_compressed.len());
  }

  #[test]
  fn test_compress_ordinarly() {
    assert_eq!(
      "000000000001010101111111100100101".to_owned(),
      compress(&"AAAAABBBBCCCDDE".to_owned()).0
    );
  }

  #[test]
  fn test_compress_non_ascii_char() {
    assert_eq!(
      "111000001".to_owned(),
      compress(&"あああいい●".to_owned()).0
    );
  }

  #[test]
  fn test_decompress_ordinary() {
    let expect = "AAAAABBBBCCCDDE".to_owned();
    let (compressed, table) = compress(&expect);
    assert_eq!(expect, decompress(compressed, &table));
  }

  #[test]
  fn test_decompress_curly() {
    let expect = "{\n\"a\": \"b\"\n}\n".to_owned();
    let (compressed, table) = compress(&expect);
    assert_eq!(expect, decompress(compressed, &table));
  }

  #[test]
  fn test_bit_of_string() {
    assert_eq!(
      (vec![0b00000000, 0b00010101, 0b01111111, 0b10010010, 0b1], 7),
      bit_of_string("000000000001010101111111100100101".to_owned())
    );
  }

  #[test]
  fn test_string_of_bit() {
    assert_eq!(
      "000000000001010101111111100100101".to_owned(),
      string_of_bit((vec![0, 21, 127, 146, 1], 7))
    );
  }

  #[test]
  fn test_by_hashmap() {
    let mut source = HashMap::new();
    source.insert("a".to_owned(), "a".to_owned());
    source.insert("b".to_owned(), "あ".to_owned());
    let source = serde_json::to_string_pretty(&source).unwrap();
    let compressed = FSCache::compress(&source);
    assert_eq!(source, FSCache::decompress(compressed));
  }

  #[test]
  fn test_real_data() {
    let mut source = String::new();
    let mut file = fs::File::open("./fixture/sample").unwrap();
    let _ = file.read_to_string(&mut source);
    let source: HashMap<String, String> = serde_json::from_str(&source).unwrap();
    let source = serde_json::to_string_pretty(&source).unwrap();
    let compressed = FSCache::compress(&source);
    assert_eq!(source, FSCache::decompress(compressed));
  }
}
