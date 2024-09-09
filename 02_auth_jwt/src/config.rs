use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
pub static ref CONFIG: Config =  Config::parse();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn parse() -> Self {
        Self {
            port: std::env::var("PORT")
                .unwrap()
                .parse::<u16>()
                .unwrap_or_default(),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap()
                .parse::<String>()
                .unwrap_or_default(),
        }
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3000,
            jwt_secret: String::from("LS6FJu8GYhDVibcHVQLdJxRSBBFchEfigekYTcmOmnIgqZtS7GFmyl"),
        }
    }
}
