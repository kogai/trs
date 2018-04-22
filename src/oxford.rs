use oxford_dictionary_api_rs::{apis, models};

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

header! { (AppId, "app_id") => [String] }
header! { (AppKey, "app_key") => [String] }

pub fn definitions(target_language: &String, query_text: &String) -> String {
  enumlate_examples(call(target_language, query_text))
}

fn call(target_language: &String, query_text: &String) -> models::RetrieveEntry {
  let mut core = Core::new().unwrap();
  let handle = core.handle();
  let client = Client::configure()
    .connector(HttpsConnector::new(4, &handle).unwrap())
    .build(&handle);
  let configure = apis::configuration::Configuration::new(client);

  let api_client = apis::client::APIClient::new(configure);
  let work = api_client
    .dictionary_entries_api()
    .entries_source_lang_word_id_get(
      target_language,
      query_text,
      env!("OXFORD_API_ID"),
      env!("OXFORD_API_KEY"),
    );

  match core.run(work) {
    Ok(json) => json,
    Err(e) => unreachable!("Perhaps JSON serialize error occurred.\n{:?}", e),
  }
}

fn enumlate_examples(json: models::RetrieveEntry) -> String {
  match json.results() {
    Some(results) => results
      .iter()
      .map(|result| {
        result
          .lexical_entries()
          .iter()
          .map(|lexical_entry| {
            let lexical_category = lexical_entry.lexical_category();
            let entries = match lexical_entry.entries() {
              Some(entries) => entries
                .iter()
                .map(|entry| match entry.senses() {
                  Some(senses) => senses
                    .iter()
                    .map(|sense| match (sense.definitions(), sense.examples()) {
                      (Some(definitions), Some(examples)) => format!(
                        "Definition: {}\n\nExamples: {}",
                        definitions.join("\n\n"),
                        examples
                          .iter()
                          .map(|example| example.text().to_owned())
                          .collect::<Vec<String>>()
                          .join("\n\n")
                      ),
                      (Some(definitions), _) => format!("Definition: {}", definitions.join("\n\n")),
                      (_, Some(examples)) => format!(
                        "Examples: {}",
                        examples
                          .iter()
                          .map(|example| example.text().to_owned())
                          .collect::<Vec<String>>()
                          .join("\n\n")
                      ),
                      (_, _) => "".to_owned(),
                    })
                    .collect::<Vec<String>>()
                    .join("\n"),
                  _ => "".to_owned(),
                })
                .collect::<Vec<String>>()
                .join("\n"),
              _ => "".to_owned(),
            };
            format!("Lexixal category [{}]\n\n{}", lexical_category, entries)
          })
          .collect::<Vec<String>>()
          .join("\n\n\n")
      })
      .collect::<Vec<String>>()
      .join("\n"),
    None => "".to_owned(),
  }
}
