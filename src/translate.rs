use std::io::Read;
use reqwest::Client;
use serde_json;

const API_TRANSLATE: &'static str = "https://www.googleapis.com/language/translate/v2";
const API_DETECT: &'static str = "/detect";

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

#[test]
fn it_should_parse_success() {
    assert_eq!(serde_json::from_str::<Translate>(r#"
    {
        "data": {
            "translations": [
                {
                    "translatedText": "hi",
                    "detectedSourceLanguage": "en"
                }
            ]
        }
    }
    "#).unwrap(), Translate::Success {
        data: Data {
            translations: vec![Translations {
                translated_text: "hi".to_owned(),
                detected_source_language: "en".to_owned(),
            }]
        }
    });
}

pub fn translate(api_key: String, target_language: String, query_text: String) -> String {
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
    
    let response = serde_json::from_str::<Translate>(&buffer).unwrap();
    match response {
        Translate::Success { data } => {
            data
                .translations
                .iter()
                .map(|t| t.translated_text.to_owned())
                .collect::<Vec<_>>()
                .join(",")

        },
        Translate::Error { error } => {
            format!("[{}]: {}", error.code, error.message)
        },
    }
}

