#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate oxford_dictionary_api_rs;

mod oxford;
mod translate;

use clap::{App, Arg};

fn main() {
    /*
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about("Translate text over google translates API")
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

    let target_language =
        value_t!(matches.value_of("target_language"), String).unwrap_or("ja".to_owned());

    let result = if matches.is_present("languages") {
        translate::language(target_language)
    } else {
        let query_words = values_t!(matches.values_of("query_text"), String).unwrap_or(vec![]);
        let query_text = query_words.join(" ");
        translate::translate(target_language, query_text)
    };
    */
    oxford::call();
    // println!("{}", result);
}
