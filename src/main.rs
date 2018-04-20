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

use clap::{App, Arg};
use std::env;

fn main() {
    let path_to_env = env::home_dir().and_then(|a| Some(a.join("trs").join(".env")));
    match path_to_env {
        Some(x) => {
            dotenv::from_path(x.as_path()).ok();
        }
        None => {}
    };

    let matches = App::new("trs")
        .version("0.1.1")
        .about("translate text over google translates API")
        .arg(
            Arg::with_name("query_text")
                .help("Set the words that translate to")
                .short("q")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("languages")
                .long("languages")
                .short("l")
                .help("See the list of languages"),
        )
        .arg(
            Arg::with_name("target_language")
                .help("Set the language in which words are translated")
                .short("t")
                .takes_value(true),
        )
        .get_matches();

    let is_languages = matches.is_present("languages");
    let target_language =
        value_t!(matches.value_of("target_language"), String).unwrap_or("ja".to_owned());

    let result = match is_languages {
        true => translate::language(target_language),
        false => {
            let query_words = values_t!(matches.values_of("query_text"), String).unwrap_or(vec![]);
            let query_text = query_words.join(" ");
            translate::translate(target_language, query_text)
        }
    };

    println!("{}", result);
}
