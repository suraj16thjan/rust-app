use std::env;

pub struct Config {
    pub bind_address: String,
}

impl Config {
    pub fn new() -> Config {
        dotenv::dotenv().ok();
        Config {
            bind_address: env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set"),
        }
    }
}
