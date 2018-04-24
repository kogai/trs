pub fn space_to_underscore(base: &String) -> String {
  base.chars().fold("".to_owned(), |acc, c| {
    if c.is_whitespace() {
      format!("{}_", acc)
    } else {
      format!("{}{}", acc, c)
    }
  })
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_space_to_underscore() {
    assert_eq!(
      space_to_underscore(&"out of the box".to_owned()),
      "out_of_the_box".to_owned()
    );
    assert_eq!(
      space_to_underscore(&"これは　箱ですか？".to_owned()),
      "これは_箱ですか？".to_owned()
    );
  }
}
