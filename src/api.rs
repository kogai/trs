use serde_json;
use std::fmt;

const GOOGLE_CLOUD_PLATFORM_API_KEY: &'static str = env!("GOOGLE_CLOUD_PLATFORM_API_KEY");
const OXFORD_API_ID: &'static str = env!("OXFORD_API_ID");
const OXFORD_API_KEY: &'static str = env!("OXFORD_API_KEY");

#[derive(Deserialize, Serialize)]
pub struct Response {
  google_clould_platform_api_key: String,
  oxford_api_id: String,
  oxford_api_key: String,
}

impl Response {
  pub fn new() -> Self {
    Response {
      google_clould_platform_api_key: GOOGLE_CLOUD_PLATFORM_API_KEY.to_owned(),
      oxford_api_id: OXFORD_API_ID.to_owned(),
      oxford_api_key: OXFORD_API_KEY.to_owned(),
    }
  }
}

impl fmt::Display for Response {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let response = serde_json::to_string_pretty(&self).unwrap();
    write!(f, "{}", response)
  }
}
