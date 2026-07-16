use std::{fmt::Debug, str::FromStr};

use crate::error::AppError;

pub fn extract_env<T>(key: &str) -> Result<T, AppError>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let env = std::env::var(key)
        .map_err(|err| AppError::ConfigError(key.to_string(), err.to_string()))?;

    let parsed = env
        .parse::<T>()
        .map_err(|err| AppError::ConfigError(key.to_string(), format!("{:?}", err)))?;

    Ok(parsed)
}
