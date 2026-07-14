
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("failed to startup app due to error: {0}")]
    StartupError(String),
}