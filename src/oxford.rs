use oxford_dictionary_api_rs::{apis, models};

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

header! { (AppId, "app_id") => [String] }
header! { (AppKey, "app_key") => [String] }

// TODO: Make fn private
pub fn call() -> models::RetrieveEntry {
  let mut core = Core::new().unwrap();
  let handle = core.handle();
  let client = Client::configure()
    .connector(HttpsConnector::new(4, &handle).unwrap())
    .build(&handle);
  let configure = apis::configuration::Configuration::new(client);

  let api_client = apis::client::APIClient::new(configure);
  let work = api_client
    .dictionary_entries_api()
    .entries_source_lang_word_id_get("en", "dog", env!("OXFORD_API_ID"), env!("OXFORD_API_KEY"));

  match core.run(work) {
    Ok(json) => json,
    Err(e) => unreachable!("Perhaps JSON serialize error occurred.\n{:?}", e),
  }
}

pub fn enumlate_examples(json: models::RetrieveEntry) -> String {
  // results[].lexicalEntries[].entries[].senses[].definitions[]
  // results[].lexicalEntries[].entries[].senses[].examples[]
  // results[].lexicalEntries[].entries[].lexicalCategory
  unimplemented!();
}
