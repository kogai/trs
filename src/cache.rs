use std::slice;
use std::collections::HashMap;

enum Tree {
  Leaf(u8),
  Node { left: Box<Tree>, right: Box<Tree> },
}

impl Tree {
  pub fn new(source: &String) -> Self {
    unimplemented!();
  }

  pub fn count(source: &String) -> Vec<([u8; 2], u8)> {
    let mut xs = source
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
        (buf, n)
      })
      .collect::<Vec<([u8; 2], u8)>>();
    xs.sort_by(|&(_, a), &(_, b)| a.cmp(&b));
    xs
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
        ([69, 0], 1),
        ([68, 0], 2),
        ([67, 0], 3),
        ([66, 0], 4),
        ([65, 0], 5),
      ],
      Tree::count(&x.to_owned())
    );
  }

  #[test]
  fn test_haffman_coding() {
    let x = "AAAAABBBBCCCDDE";
    assert_eq!("", x);
  }
}
