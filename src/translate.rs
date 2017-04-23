use std::io::Read;
use reqwest::Client;
use serde_json;

const API_TRANSLATE: &'static str = "https://www.googleapis.com/language/translate/v2";
// const API_DETECT: &'static str = "https://www.googleapis.com/language/translate/v2/detect";

#[derive(Debug, Serialize, Deserialize)]
struct ErrorReason {
    domain: String,
    reason: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Errors {
    errors: Vec<ErrorReason>,
    code: i32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslateResponseError {
    error: Errors,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Translations {
    translated_text: String,
    detected_source_language: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    translations: Vec<Translations>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslateResponseSuccess {
    data: Data,
}

pub fn translate(api_key: String, target_language: &str, query_text: &str) -> String {
    let http_client = Client::new().expect("Create HTTP client is failed");
    let url = format!("{}?q={}&target={}&format=text&key={}",
                      API_TRANSLATE,
                      query_text,
                      target_language,
                      api_key);
    let mut buffer = String::new();

    http_client.get(url.as_str())
        .send()
        .expect("send Request failed")
        .read_to_string(&mut buffer)
        .expect("read response failed");

    let response = serde_json::from_str::<TranslateResponseSuccess>(&buffer);
    match response {
        Ok(r) => {
            r.data
                .translations
                .iter()
                .map(|t| t.translated_text.to_owned())
                .collect::<Vec<_>>()
                .join(",")
        }
        Err(e) => {
            let response_error = serde_json::from_str::<TranslateResponseError>(&buffer);
            match response_error {
                Ok(e) => format!("[{}]: {}", e.error.code, e.error.message),
                Err(_) => format!("{:?}", e),
            }
        }
    }
}

