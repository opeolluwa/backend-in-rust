use serde::{Deserialize, Serialize};

lazy_static::lazy_static! {
  pub static ref  CONFIG : Config  = Config::parse();
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_application_port")]
    pub port: u16,
    pub jwt_signing_key: String,
    pub database_connection_string: String,
}

impl Config {
    pub fn parse() -> Self {
        let config_path =
            std::env::var("CONFIG_FILE").expect("Couldn't read configuration file path env");

        let config_yml = std::fs::read_to_string(&config_path)
            .expect("Unable to read the content of the application config file");
        eprintln!(" Config file path {}", &config_path);

        // extract to yml \
        let config =
            serde_yml::from_str(&config_yml).expect("Unable to extract application configuration");

        config
    }

    pub fn default_application_port() -> u16 {
        3000
    }
}
