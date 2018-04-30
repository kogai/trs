use oxford_dictionary_api_rs::{apis, models};

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use std::fmt::{self, Display, Formatter};
use termion::{color, style};
use utils::space_to_underscore;

pub fn definitions(query_words: Vec<String>) -> String {
  match call(query_words) {
    Ok(result) => enumlate_examples(result)
      .iter()
      .map(|d| format!("{}", d))
      .collect::<Vec<String>>()
      .join("\n\n"),
    Err(reason) => reason,
  }
}

fn call(query_words: Vec<String>) -> Result<models::RetrieveEntry, String> {
  let query_text = space_to_underscore(&query_words.join(" "));
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
      &query_text,
      env!("OXFORD_API_ID"),
      env!("OXFORD_API_KEY"),
    );

  match core.run(work) {
    Ok(json) => Ok(json),
    Err(_) => Err(format!(
      "{}●{} [{}{}{}] was not found in Oxford dictionary",
      color::Fg(color::Green),
      color::Fg(color::White),
      style::Bold,
      &query_words.join(" "),
      style::Reset
    )),
  }
}

type DefinitionAndExample = (Vec<String>, Vec<String>);

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
      "{}●{} Lexical category [{}{}{}]\n{}",
      color::Fg(color::Green),
      color::Fg(color::White),
      style::Bold,
      self.lexical_category,
      style::Reset,
      self
        .definition_and_examples
        .iter()
        .map(|&(ref ds, ref es)| {
          let d = ds.join("\n    ");
          let e = es.join("\n    ");
          if e.len() == 0 {
            format!(
              r"
  Definition:
    {}",
              ds.join("\n")
            )
          } else {
            format!(
              r"
  Definition:
    {}
  Example:
    {}",
              d, e
            )
          }
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
                        definitions.to_owned(),
                        examples
                          .iter()
                          .map(|example| example.text().to_owned())
                          .collect::<Vec<String>>(),
                      ),
                      (Some(definitions), _) => (definitions.to_owned(), Vec::new()),
                      (_, Some(examples)) => (
                        Vec::new(),
                        examples
                          .iter()
                          .map(|example| example.text().to_owned())
                          .collect::<Vec<String>>(),
                      ),
                      (_, _) => (Vec::new(), Vec::new()),
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
