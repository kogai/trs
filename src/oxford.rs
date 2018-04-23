use oxford_dictionary_api_rs::{apis, models};

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use std::fmt::{self, Display, Formatter};

header! { (AppId, "app_id") => [String] }
header! { (AppKey, "app_key") => [String] }

pub fn definitions(query_text: &String) -> String {
  enumlate_examples(call(query_text))
    .iter()
    .map(|d| format!("{}", d))
    .collect::<Vec<String>>()
    .join("\n\n")
}

fn call(query_text: &String) -> models::RetrieveEntry {
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
      "en",
      query_text,
      env!("OXFORD_API_ID"),
      env!("OXFORD_API_KEY"),
    );

  match core.run(work) {
    Ok(json) => json,
    Err(e) => unreachable!("Perhaps JSON serialize error occurred.\n{:?}", e),
  }
}

type DefinitionAndExample = (Option<String>, Option<String>);

struct Definition {
  lexical_category: String,
  definition_and_examples: Vec<DefinitionAndExample>,
}

impl Definition {
  fn new(lexical_category: &String, definition_and_examples: Vec<DefinitionAndExample>) -> Self {
    Definition {
      lexical_category: lexical_category.to_owned(),
      definition_and_examples,
    }
  }
}

impl Display for Definition {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "●Lexixal category [{}]\n{}",
      self.lexical_category,
      self
        .definition_and_examples
        .iter()
        .map(|d_and_e| match d_and_e {
          &(Some(ref d), Some(ref e)) => format!(
            r"
Definition: {}
Example: {}",
            d, e
          ),
          &(Some(ref d), None) => format!(
            r"
Definition: {}",
            d
          ),
          &(None, Some(ref e)) => format!(
            r"
Example: {}",
            e
          ),
          _ => format!(""),
        })
        .collect::<Vec<String>>()
        .join("\n")
    )
  }
}

fn enumlate_examples(json: models::RetrieveEntry) -> Vec<Definition> {
  match json.results() {
    Some(results) => results
      .iter()
      .flat_map(|result| {
        result
          .lexical_entries()
          .iter()
          .map(|lexical_entry| {
            let lexical_category = lexical_entry.lexical_category();
            let entries = match lexical_entry.entries() {
              Some(entries) => entries
                .iter()
                .flat_map(|entry| match entry.senses() {
                  Some(senses) => senses
                    .iter()
                    .map(|sense| match (sense.definitions(), sense.examples()) {
                      (Some(definitions), Some(examples)) => (
                        Some(definitions.join("\n")),
                        Some(
                          examples
                            .iter()
                            .map(|example| example.text().to_owned())
                            .collect::<Vec<String>>()
                            .join("\n"),
                        ),
                      ),
                      (Some(definitions), _) => (Some(definitions.join("\n")), None),
                      (_, Some(examples)) => (
                        None,
                        Some(
                          examples
                            .iter()
                            .map(|example| example.text().to_owned())
                            .collect::<Vec<String>>()
                            .join("\n"),
                        ),
                      ),
                      (_, _) => (None, None),
                    })
                    .collect(),
                  _ => Vec::new(),
                })
                .collect(),
              _ => Vec::new(),
            };
            Definition::new(lexical_category, entries)
          })
          .collect::<Vec<Definition>>()
      })
      .collect(),
    None => Vec::new(),
  }
}
