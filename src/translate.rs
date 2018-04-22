use reqwest::Client;
use serde_json;
use std::io::Read;

const ENDPOINT_GOOGLE: &'static str = "https://translation.googleapis.com/language/translate/v2";
const API_LANGUAGES: &'static str = "/languages";
const API_KEY: &'static str = env!("GOOGLE_CLOUD_PLATFORM_API_KEY");

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ErrorReason {
    domain: String,
    reason: String,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Errors {
    errors: Vec<ErrorReason>,
    code: i32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Translations {
    translated_text: String,
    detected_source_language: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Data {
    translations: Vec<Translations>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Translate {
    Success { data: Data },
    Error { error: Errors },
}

enum Method {
    Get,
    Post,
}

fn request(path: String, method: Method) -> String {
    let http_client = Client::new().expect("Create HTTP client is failed");
    let url = format!("{}{}&key={}", ENDPOINT_GOOGLE, path, API_KEY);
    let mut buffer = String::new();
    match method {
        Method::Get => http_client.get(url.as_str()),
        Method::Post => http_client.post(url.as_str()),
    }.send()
        .expect("send Request failed")
        .read_to_string(&mut buffer)
        .expect("read response failed");

    buffer
}

pub fn translate(target_language: &String, query_text: &String) -> String {
    let path = format!("?q={}&target={}&format=text", query_text, target_language);
    let buffer = request(path, Method::Post);
    let response = serde_json::from_str::<Translate>(&buffer).unwrap();

    match response {
        Translate::Success { data } => data.translations
            .iter()
            .map(|t| t.translated_text.to_owned())
            .collect::<Vec<_>>()
            .join(","),
        Translate::Error { error } => format!("[{}]: {}", error.code, error.message),
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Languages {
    language: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DataLanguage {
    languages: Vec<Languages>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Language {
    Success { data: DataLanguage },
    Error { error: Errors },
}

pub fn language(target_language: String) -> String {
    let path = format!("{}?target={}", API_LANGUAGES, target_language);
    let buffer = request(path, Method::Get);

    let response = serde_json::from_str::<Language>(&buffer).unwrap();

    match response {
        Language::Success { data } => data.languages
            .iter()
            .map(|l| format!("{}: {}", l.name, l.language))
            .collect::<Vec<_>>()
            .join("\n"),
        Language::Error { error } => format!("[{}]: {}", error.code, error.message),
    }
}
