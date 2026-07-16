use secrecy::SecretString;

use crate::{error::AppError, helper::extract_env};

#[derive(Debug)]
pub struct AppConfig {
    pub port: u16,
    pub database_url: SecretString,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let port = extract_env::<u16>("PORT")?;
        let database_url = SecretString::from(extract_env::<String>("DATABASE_URL")?);

        Ok(Self { port, database_url })
    }
}
