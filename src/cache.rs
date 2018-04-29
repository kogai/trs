use std::collections::HashMap;

/* TODO: It would be better to implement probability helper to treat it easily
struct Probability {
  raw: char,
}
*/

#[derive(Debug, Clone, PartialEq)]
enum HaffmanTree {
  Leaf(([u8; 2], u8)),
  Node {
    zero: Box<HaffmanTree>,
    one: Box<HaffmanTree>,
    probability: u8,
  },
}

impl HaffmanTree {
  fn get_codes(&self, code: &String, mut code_table: &mut HashMap<char, String>) {
    use self::HaffmanTree::*;

    match self {
      &Leaf((codes, _)) => {
        let raw_code = *codes.first().unwrap();
        code_table.insert(char::from(raw_code), code.to_owned());
      }
      &Node {
        ref zero, ref one, ..
      } => {
        zero.get_codes(&format!("{}0", code), &mut code_table);
        one.get_codes(&format!("{}1", code), &mut code_table);
      }
    }
  }

  fn get_table(&self) -> HashMap<char, String> {
    use self::HaffmanTree::*;

    let mut code_table = HashMap::new();
    match self {
      &Leaf((codes, _)) => {
        let code = *codes.first().unwrap();
        code_table.insert(char::from(code), "0".to_owned());
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

  fn get_probability(&self) -> u8 {
    use self::HaffmanTree::*;
    match self {
      &Leaf((_, p)) => p,
      &Node { probability, .. } => probability,
    }
  }

  fn new(source: &String) -> Self {
    let leafs: Vec<HaffmanTree> = Self::count(source)
      .iter()
      .map(|&(_, c)| HaffmanTree::Leaf(c))
      .collect();
    Self::build_tree(leafs)
  }

  fn build_tree(trees: Vec<Self>) -> Self {
    match trees.len() {
      0 => unreachable!("Could not found trees, something become wrong."),
      1 => trees.first().unwrap().clone(),
      _ => {
        let mut trees = trees;
        trees.sort_by(|a, b| a.get_probability().cmp(&b.get_probability()));
        let mut remains = trees.split_off(2);
        let small_fst = trees.get(0).unwrap();
        let small_snd = trees.get(1).unwrap();
        let new_tree = HaffmanTree::Node {
          zero: Box::new(small_snd.clone()),
          one: Box::new(small_fst.clone()),
          probability: small_fst.get_probability() + small_snd.get_probability(),
        };
        remains.push(new_tree);
        HaffmanTree::build_tree(remains)
      }
    }
  }

  fn count(source: &String) -> Vec<(char, ([u8; 2], u8))> {
    let mut length_of_chars = source
      .chars()
      .fold(HashMap::new(), |mut acc: HashMap<char, (char, u8)>, c| {
        if let Some(&(_, n)) = acc.get(&c) {
          acc.insert(c, (c, n + 1));
        } else {
          acc.insert(c, (c, 1));
        };
        acc
      })
      .into_iter()
      .map(|(_, (c, n))| {
        let mut buf = [0; 2];
        c.encode_utf8(&mut buf);
        (c, (buf, n))
      })
      .collect::<Vec<(char, ([u8; 2], u8))>>();
    length_of_chars.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    length_of_chars
  }
}

fn compress(source: &String) -> String {
  let table = HaffmanTree::new(source).get_table();
  source.chars().fold("".to_owned(), |acc, c| {
    let code = table
      .get(&c)
      .expect("Haffman-table should always to code all characters");
    format!("{}{}", acc, code)
  })
}

fn decompress(source: &String, table: &HashMap<char, String>) -> String {
  let invert_table = table
    .into_iter()
    .map(|(k, v)| (v.clone(), k.clone()))
    .collect::<HashMap<String, char>>();

  fn decompress_impl(
    code_buf: &String,
    result_buf: &String,
    source: &String,
    table: &HashMap<String, char>,
  ) -> String {
    if source.len() == 0 && code_buf.len() == 0 {
      return result_buf.to_owned();
    };
    match table.get(code_buf) {
      Some(next_char) => decompress_impl(
        &"".to_owned(),
        &format!("{}{}", result_buf, next_char),
        source,
        table,
      ),
      None => {
        let (c, next) = source.split_at(1);
        decompress_impl(
          &format!("{}{}", code_buf, c),
          result_buf,
          &next.to_owned(),
          table,
        )
      }
    }
  }
  decompress_impl(&"".to_owned(), &"".to_owned(), source, &invert_table)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_haffman_count() {
    let x = "AAAAABBBBCCCDDE".to_owned();
    assert_eq!(
      vec![
        ('E', ([69, 0], 1)),
        ('D', ([68, 0], 2)),
        ('C', ([67, 0], 3)),
        ('B', ([66, 0], 4)),
        ('A', ([65, 0], 5)),
      ],
      HaffmanTree::count(&x.to_owned())
    );
  }

  #[test]
  fn test_haffman_simple_tree() {
    use self::HaffmanTree::*;
    let x = HaffmanTree::new(&"AAB".to_owned());
    println!("{:#?}", x);
    assert_eq!(
      Node {
        zero: Box::new(Leaf(([65, 0], 2))),
        one: Box::new(Leaf(([66, 0], 1))),
        probability: 3,
      },
      x
    );
  }

  #[test]
  fn test_haffman_bit_complecated_tree() {
    use self::HaffmanTree::*;
    let x = HaffmanTree::new(&"AAABBC".to_owned());
    assert_eq!(
      Node {
        zero: Box::new(Node {
          zero: Box::new(Leaf(([66, 0], 2))),
          one: Box::new(Leaf(([67, 0], 1))),
          probability: 3,
        }),
        one: Box::new(Leaf(([65, 0], 3))),
        probability: 6,
      },
      x
    );
  }

  #[test]
  fn test_haffman_tree() {
    use self::HaffmanTree::*;
    let x = HaffmanTree::new(&"AAAAABBBBCCCDDE".to_owned());
    assert_eq!(
      Node {
        zero: Box::new(Node {
          zero: Box::new(Leaf(([65, 0], 5))),
          one: Box::new(Leaf(([66, 0], 4))),
          probability: 9,
        }),
        one: Box::new(Node {
          zero: Box::new(Node {
            zero: Box::new(Leaf(([68, 0], 2))),
            one: Box::new(Leaf(([69, 0], 1))),
            probability: 3,
          }),
          one: Box::new(Leaf(([67, 0], 3))),
          probability: 6,
        }),
        probability: 15,
      },
      x
    );
  }

  #[test]
  fn test_haffman_table() {
    let x = HaffmanTree::new(&"AAAAABBBBCCCDDE".to_owned()).get_table();
    /*
    1000010
    1000011
    1000101
    1000001
    1000100
     */
    assert_eq!(
      vec![
        ('A', "00".to_owned()),
        ('B', "01".to_owned()),
        ('C', "11".to_owned()),
        ('D', "100".to_owned()),
        ('E', "101".to_owned()),
      ].into_iter()
        .collect::<HashMap<char, String>>(),
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
    assert!((compressed.len() as f32 / not_compressed.len() as f32) < 0.5);
    assert!(compressed.len() < not_compressed.len());
  }

  #[test]
  fn test_compress() {
    assert_eq!(
      "000000000001010101111111100100101".to_owned(),
      compress(&"AAAAABBBBCCCDDE".to_owned())
    );
  }

  #[test]
  fn test_decompress() {
    let expect = "AAAAABBBBCCCDDE".to_owned();
    let table = HaffmanTree::new(&expect).get_table();
    assert_eq!(
      expect,
      decompress(&"000000000001010101111111100100101".to_owned(), &table)
    );
  }
}
