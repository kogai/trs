use reqwest::Client;
use serde_json;
use std::io::Read;

const ENDPOINT: &'static str = "https://od-api.oxforddictionaries.com/api/v1";
const ID: &'static str = env!("OXFORD_API_ID");
const KEY: &'static str = env!("OXFORD_API_KEY");
