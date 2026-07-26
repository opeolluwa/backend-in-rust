use std::net::{Ipv4Addr, SocketAddrV4};



use crate::{
    config::{env::load_env, logger::init_logger},
    errors::AppError,

    router,
};

pub async fn run() -> Result<(), AppError> {
    let config = load_env()?;

    init_logger(&config);

    let app = router::routes().layer(tower_http::trace::TraceLayer::new_for_http());

    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, config.port);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    tracing::info!("ignition started on http://{addr}");

    axum::serve(listener, app)
        .await
        .map_err(|err| AppError::StartupError(err.to_string()))?;

    Ok(())
}
