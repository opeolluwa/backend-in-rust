use lib_hi_axum::{app, errors::AppError};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    app::run().await?;
    Ok(())
}
