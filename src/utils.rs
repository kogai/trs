use std::env;

pub fn get_env(kind: &str) -> String {
    env::var(kind).expect(&format!("Missing environment parameter {}", kind))
}
