enum Tree {
  Leaf(u8),
  Node { left: Box<Tree>, right: Box<Tree> },
}

impl Tree {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_haffman_coding() {
    let n: u8 = 100;
    assert_eq!(100, n);
  }
}
