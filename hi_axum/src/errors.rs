#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("failed to start up the app due to {0}")]
    StartupError(String),
    #[error("invalid env was passed {0}")]
    InvalidEnv(String),
    #[error("failed to extract {0} due to {1}")]
    EnvExtractionError(String, String),
    #[error("failed to parse {0} due to {1}")]
    EnvParsingError(String, String),
}
