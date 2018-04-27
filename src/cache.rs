use std::slice;
use std::collections::HashMap;

#[derive(Debug)]
enum HaffmanTree {
  Leaf(u8),
  Node {
    left: Box<HaffmanTree>,
    right: Box<HaffmanTree>,
  },
}

impl HaffmanTree {
  pub fn new(source: &String) -> Self {
    unimplemented!();
  }

  pub fn count(source: &String) -> HashMap<char, ([u8; 2], u8)> {
    source
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
      .collect()
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
      ].iter()
        .cloned()
        .collect::<HashMap<char, ([u8; 2], u8)>>(),
      HaffmanTree::count(&x.to_owned())
    );
  }

  #[test]
  fn test_haffman_tree() {
    let x = HaffmanTree::new(&"AAAAABBBBCCCDDE".to_owned());
    println!("{:?}", x);
    // assert_eq!("", x);
  }
}
