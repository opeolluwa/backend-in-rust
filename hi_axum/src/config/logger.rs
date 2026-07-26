use crate::config::env::{AppEnv, Environment};

pub fn init_logger(config: &AppEnv) {
    let log_levels = match config.environment {
        Environment::Development => tracing::Level::DEBUG,
        Environment::Production => tracing::Level::INFO,
    };
    
    tracing_subscriber::fmt()
        .with_max_level(log_levels)
        .compact()
        .init();
}
