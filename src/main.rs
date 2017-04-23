extern crate dotenv;

mod utils;

use dotenv::dotenv;
use utils::get_env;

fn main() {
    dotenv().ok();
    let api_key = get_env("GOOGLE_CLOUD_PLATFORM_API_KEY");
    println!("Hello, world! {}", api_key);
}

