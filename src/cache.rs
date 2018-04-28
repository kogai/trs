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
  fn get_probability(&self) -> u8 {
    use self::HaffmanTree::*;
    match self {
      &Leaf((_, p)) => p,
      &Node { probability, .. } => probability,
    }
  }

  pub fn new(source: &String) -> Self {
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

  // 1000010
  // 1000011
  // 1000101
  // 1000001
  // 1000100
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
}
