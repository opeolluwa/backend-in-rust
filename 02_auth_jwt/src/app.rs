use std::net::{Ipv4Addr, SocketAddrV4};

use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;

use crate::config::CONFIG;
use crate::routes::router;

pub struct AuthJwt;

impl AuthJwt {
    pub async fn run() -> Result<(), anyhow::Error> {
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, CONFIG.port);

        let app = router();
        let listener = TcpListener::bind(addr).await?;

        let database_connection_options = ConnectOptions::new(&CONFIG.database_connection_string);
        let _ = Database::connect(database_connection_options).await?;

        axum::serve(listener, app).await?;
        Ok(())
    }
}
