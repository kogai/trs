use std::collections::HashMap;

/* TODO: It would be better to implement probability helper to treat it easily
struct Probability {
  raw: char,
}
*/

#[derive(Debug)]
enum HaffmanTree {
  Leaf(([u8; 2], u8)),
  Node {
    zero: Box<HaffmanTree>,
    one: Box<HaffmanTree>,
    probability: u8,
  },
}

impl HaffmanTree {
  pub fn new(source: &String) -> Self {
    let length_of_chars = Self::count(source);
    match (length_of_chars.split_first(), length_of_chars.split_first()) {
      (Some((&(_, first), _)), Some((&(_, second), xs))) => {
        let root = if first.1 >= second.1 {
          HaffmanTree::Node {
            zero: Box::new(HaffmanTree::Leaf(first)),
            one: Box::new(HaffmanTree::Leaf(second)),
            probability: first.1 + second.1,
          }
        } else {
          HaffmanTree::Node {
            zero: Box::new(HaffmanTree::Leaf(second)),
            one: Box::new(HaffmanTree::Leaf(first)),
            probability: first.1 + second.1,
          }
        };
        xs.iter().fold(root, |acc, &(_, x)| {
          acc.insert(x)
          // println!("{:?}", acc);
          // unimplemented!();
        })
      }
      (Some((&(_, first), _)), None) => HaffmanTree::Leaf(first),
      (_, _) => unreachable!("There not exists any characters: [{}]", source),
    }
  }

  fn insert(&self, x: ([u8; 2], u8)) -> Self {
    let (codes, count) = x;
    unimplemented!();
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
    length_of_chars.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    length_of_chars
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_haffman_count() {
    let x = "AAAAABBBBCCCDDE".to_owned();
    assert_eq!(
      vec![
        ('A', ([65, 0], 5)),
        ('B', ([66, 0], 4)),
        ('C', ([67, 0], 3)),
        ('D', ([68, 0], 2)),
        ('E', ([69, 0], 1)),
      ],
      HaffmanTree::count(&x.to_owned())
    );
  }

  #[test]
  fn test_haffman_tree() {
    let x = HaffmanTree::new(&"AAAAABBBBCCCDDE".to_owned());
    println!("{:?}", x);
    // 1000010
    // 1000011
    // 1000101
    // 1000001
    // 1000100
    // assert_eq!("", x);
  }
}
