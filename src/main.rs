#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate clap;

mod translate;
mod utils;

use dotenv::dotenv;
use clap::{Arg, App};
use translate::translate;
use utils::get_env;

fn main() {
    dotenv().ok();
    let api_key = get_env("GOOGLE_CLOUD_PLATFORM_API_KEY");

    let matches = App::new("trslt")
        .version("0.1")
        .about("translate text over google translates API")

        .arg(Arg::with_name("query_text")
                 .help("Set the words that translate to")
                 .required(true))
        .arg(Arg::with_name("target_language")
                 .help("Set the language in which words are translated"))
        .get_matches();

    let target_language = matches.value_of("target_language").unwrap_or("ja");
    let query_text = matches.value_of("query_text").unwrap();
    let result = translate(api_key, target_language, query_text);
    println!("{}", result);
}