#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod translate;
mod utils;

use std::env;
use clap::{Arg, App};
use translate::translate;
use utils::get_env;

fn main() {
    let path_to_env = env::home_dir().and_then(|a| Some(a.join("trs").join(".env")));
    match path_to_env {
        Some(x) => {
            dotenv::from_path(x.as_path()).ok();
        },
        None => {},
    };

    let api_key = get_env("GOOGLE_CLOUD_PLATFORM_API_KEY");
    let matches = App::new("trs")
        .version("0.1.1")
        .about("translate text over google translates API")
        .arg(Arg::with_name("query_text")
                 .help("Set the words that translate to")
                 .short("q")
                 .takes_value(true)
                 .multiple(true)
                 .required(true))
        .arg(Arg::with_name("target_language")
                 .help("Set the language in which words are translated")
                 .short("t")
                 .takes_value(true))
        .get_matches();

    let target_language = value_t!(matches.value_of("target_language"), String).unwrap_or("ja".to_owned());
    let query_words = values_t!(matches.values_of("query_text"), String).unwrap_or(vec![]);
    let query_text = query_words.join(" ");

    let result = translate(api_key, target_language, query_text);
    println!("{}", result);
}