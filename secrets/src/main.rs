use std::net::{Ipv4Addr, SocketAddrV4};

use axum::{Router, routing::get};
use lib_secrets::{config::AppConfig, error::AppError};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let app_config = AppConfig::from_env()?;

    let app = Router::new().route("/", get(say_hello));

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, app_config.port);
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    println!("http server running on http://{}", addr);

    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    Ok(())
}

async fn say_hello() -> String {
    "hello, World".to_string()
}
