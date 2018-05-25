#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate flame;
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

use clap::{App, Arg, SubCommand};
use std::io::{self, Write};
use std::process::exit;
use utils::span_of;

fn translate_between(
    query_words: Vec<String>,
    target_language: String,
    namespace: cache::Namespace,
    fs_cache: &mut cache::FSCache,
) -> String {
    let query_text = query_words.join(" ");
    match fs_cache.get(&namespace, &query_text) {
        Some(definitions) => definitions,
        None => {
            let new_def = translate::translate(&target_language, &query_text);
            fs_cache.set(&namespace, &query_text, &new_def);
            new_def
        }
    }
}

fn run() {
    let arg_of_dictionary = Arg::with_name("dictionary")
        .index(1)
        .takes_value(true)
        .multiple(true);

    let arg_of_to = Arg::with_name("to-target-language")
        .index(1)
        .takes_value(true)
        .multiple(true);

    let arg_of_from = Arg::with_name("from-target-language")
        .index(1)
        .takes_value(true)
        .multiple(true);

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about("CLI tool for English learners")
        .arg(
            Arg::with_name("languages")
                .long("languages")
                .short("l")
                .help("See the list of languages"),
        )
        .arg(Arg::with_name("cat").help("Show current cache").short("c"))
        .arg(
            Arg::with_name("change-language")
                .help("Change the language correspoinding to english")
                .short("C")
                .takes_value(true),
        )
        .subcommands(vec![
            SubCommand::with_name("dictionary")
                .about("See formal English definition of the words")
                .arg(&arg_of_dictionary),
            SubCommand::with_name("d")
                .about("Alias for dictionary")
                .arg(arg_of_dictionary),
            SubCommand::with_name("from")
                .about("Set the words that translate from target language to english")
                .arg(&arg_of_from),
            SubCommand::with_name("f")
                .about("Alias for from")
                .arg(arg_of_from),
            SubCommand::with_name("to")
                .about("Set the words that translate to")
                .arg(&arg_of_to),
            SubCommand::with_name("t")
                .about("Alias for to")
                .arg(arg_of_to),
        ])
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

    if matches.is_present("cat") {
        let cache = fs_cache.get_all();
        println!("{}", cache);
        exit(0);
    };

    let result = match matches.subcommand() {
        ("dictionary", Some(cmd)) | ("d", Some(cmd)) => {
            let namespace = cache::Namespace::Dictionary;
            let query_words = values_t!(cmd.values_of("dictionary"), String).unwrap_or(vec![]);
            println!("{:#?}", query_words);
            let escaped_query_words = utils::space_to_underscore(&query_words.join(" "));
            let definitions = match fs_cache.get(&namespace, &escaped_query_words) {
                Some(definitions) => definitions,
                None => {
                    let new_def = oxford::definitions(query_words);
                    fs_cache.set(&namespace, &escaped_query_words, &new_def);
                    new_def
                }
            };
            definitions
        }
        ("to", Some(cmd)) | ("t", Some(cmd)) => {
            let query_words =
                values_t!(cmd.values_of("to-target-language"), String).unwrap_or(vec![]);
            let target_language = default_language;
            translate_between(
                query_words,
                target_language,
                cache::Namespace::Translate,
                &mut fs_cache,
            )
        }
        ("from", Some(cmd)) | ("f", Some(cmd)) => {
            let query_words =
                values_t!(cmd.values_of("from-target-language"), String).unwrap_or(vec![]);
            let target_language = "en".to_owned();
            translate_between(
                query_words,
                target_language,
                cache::Namespace::Translate,
                &mut fs_cache,
            )
        }
        _ => unreachable!(),
    };

    let _ = io::stdout().write(result.as_bytes());
    let _ = span_of("garbage_collect", || fs_cache.garbage_colloect());
}

fn main() {
    span_of("run", || run());
    #[cfg(debug_assertions)]
    flame::dump_stdout();
}
