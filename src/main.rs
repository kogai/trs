#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate oxford_dictionary_api_rs;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate termion;
extern crate tokio_core;

mod cache;
mod oxford;
mod translate;
mod utils;

use clap::{App, Arg};
use std::process::exit;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about("CLI for English learners")
        .arg(
            Arg::with_name("languages")
                .long("languages")
                .short("l")
                .help("See the list of languages"),
        )
        .arg(Arg::with_name("cat").help("Show current cache"))
        .arg(
            Arg::with_name("dictionary")
                .help("See formal English definition of the words")
                .short("d")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("change-language")
                .help("Change the language correspoinding to english")
                .short("c")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("from-target-language")
                .help("Set the words that translate from target language to english")
                .short("f")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("to-target-language")
                .help("Set the words that translate to")
                .short("t")
                .takes_value(true)
                .multiple(true),
        )
        .get_matches();

    let mut fs_cache = cache::FSCache::new();
    let default_language = fs_cache.get_language();

    if matches.is_present("languages") {
        let result = translate::language(&default_language);
        println!("{}", result);
        exit(0);
    }

    if matches.is_present("change-language") {
        let language = value_t!(matches, "change-language", String).unwrap();
        fs_cache.set_language(&language);
        exit(0);
    }

    if matches.is_present("dictionary") {
        let namespace = cache::Namespace::Dictionary;
        let query_words = values_t!(matches.values_of("dictionary"), String).unwrap_or(vec![]);
        let escaped_query_words = utils::space_to_underscore(&query_words.join(" "));
        let definitions = match fs_cache.get(&namespace, &escaped_query_words) {
            Some(definitions) => definitions,
            None => {
                let new_def = oxford::definitions(query_words);
                fs_cache.set(&namespace, &escaped_query_words, &new_def);
                new_def
            }
        };
        println!("{}", definitions);
        exit(0);
    };

    if matches.is_present("cat") {
        let cache = fs_cache.get_all();
        println!("{}", cache);
        exit(0);
    };

    let namespace = cache::Namespace::Translate;
    let (query_words, target_language) = if matches.is_present("to-target-language") {
        (
            values_t!(matches.values_of("to-target-language"), String).unwrap_or(vec![]),
            default_language,
        )
    } else if matches.is_present("from-target-language") {
        (
            values_t!(matches.values_of("from-target-language"), String).unwrap_or(vec![]),
            "en".to_owned(),
        )
    } else {
        unreachable!()
    };

    let query_text = query_words.join(" ");
    let translated = match fs_cache.get(&namespace, &query_text) {
        Some(definitions) => definitions,
        None => {
            let new_def = translate::translate(&target_language, &query_text);
            fs_cache.set(&namespace, &query_text, &new_def);
            new_def
        }
    };
    println!("{}", translated);
    fs_cache.gabadge_collect();
}
