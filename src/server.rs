extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod api;

fn main() {
  let res = api::Response::new();
  println!("{}", res);
}
