use reqwest::Client;
use serde_json;
use std::io::Read;
use hyper::header::{ContentType, Headers};

const ENDPOINT: &'static str = "https://od-api.oxforddictionaries.com/api/v1";
// const APIID: &'static str = env!("OXFORD_API_ID");
// const API_KEY: &'static str = env!("OXFORD_API_KEY");

header! { (AppId, "app_id") => [String] }
header! { (AppKey, "app_key") => [String] }

enum Request {
  Entry(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
  results: Vec<Result>,
}
// pub enum Response {
//   Entry { results: Vec<Result> },
// }

// fn call(req: Request) -> Response {
pub fn call() -> Response {
  let mut headers = Headers::new();
  headers.set(ContentType::json());
  headers.set(AppId(env!("OXFORD_API_ID").to_string()));
  headers.set(AppKey(env!("OXFORD_API_KEY").to_string()));
  let client = Client::builder().default_headers(headers).build().unwrap();
  let mut buf = String::new();
  let _ = client
    .get(format!("{}/entries/en/dog", ENDPOINT).as_str())
    .send()
    .unwrap()
    .read_to_string(&mut buf);
  // println!("{:?}", client);
  println!("{:?}", serde_json::from_str::<Response>(&buf));
  unimplemented!();
  // let url = format!("{}{}&key={}", ENDPOINT, path, API_KEY);
  // let mut buffer = String::new();
  // match method {
  //   Method::Get => http_client.get(url.as_str()),
  //   Method::Post => http_client.post(url.as_str()),
  // }.send()
  //   .expect("send Request failed")
  //   .read_to_string(&mut buffer)
  //   .expect("read response failed");

  // buffer
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Result {
  id: String,
  language: String,
  lexical_entries: Vec<LexicalEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LexicalEntry {
  entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
  senses: Vec<Sense>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sense {
  definitions: Option<Vec<String>>,
  domains: Option<Vec<String>>,
  examples: Option<Vec<Example>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Example {
  text: String,
}
