use std::str::FromStr;

use crate::{errors::AppError, utils::extract_env};

#[derive(Debug)]
pub struct AppEnv {
    pub port: u16,
    pub environment: Environment,
}

pub fn load_env() -> Result<AppEnv, AppError> {
    dotenvy::dotenv().ok();
    AppEnv::parse()
}

impl AppEnv {
    fn parse() -> Result<Self, AppError> {
        let port = extract_env::<u16>("PORT")?;
        let environment = extract_env::<Environment>("ENVIRONMENT")?;

        Ok(Self { port, environment })
    }
}

#[derive(Debug)]
pub enum Environment {
    Development,
    Production,
}

impl FromStr for Environment {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" | "development" => Ok(Environment::Development),
            "prod" | "production" => Ok(Environment::Production),
            _ => Err(AppError::InvalidEnv(s.to_string())),
        }
    }
}
