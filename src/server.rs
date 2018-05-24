extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod api;
use std::io::{stdin, stdout, Read, Write};

fn main() {
  let mut buf = Vec::new();
  let _ = stdin().read_to_end(&mut buf).unwrap();
  println!("{:#?}", &buf);
  let _ = stdout().write(buf.as_slice()).unwrap();

  let res = api::Response::new();
  println!("{}", res);
}
