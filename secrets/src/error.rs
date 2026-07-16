
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("failed to startup app due to error: {0}")]
    StartupError(String),
    #[error("failed to read {0} due to error: {1}")]
    ConfigError(String, String),
}