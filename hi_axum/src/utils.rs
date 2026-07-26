use std::{fmt::Display, str::FromStr};


use crate::errors::AppError;

pub fn extract_env<T>(key: &str) -> Result<T, AppError>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let env = std::env::var(key)
        .map_err(|err| AppError::EnvExtractionError(key.to_string(), err.to_string()))?;

    let parsed = env
        .parse::<T>()
        .map_err(|err| AppError::EnvParsingError(key.to_string(), err.to_string()))?;

    Ok(parsed)
}
