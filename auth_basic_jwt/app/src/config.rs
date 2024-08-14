use std::env::var;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn parse() -> Self {
        Self {
            database_url: var("DATABASE_URL").expect("error parsing database URL"),
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: Default::default(),
        }
    }
}
