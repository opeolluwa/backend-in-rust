use std::net::{Ipv4Addr, SocketAddrV4};

use sea_orm::{ConnectOptions, Database};
use tokio::net::TcpListener;

use crate::config::CONFIG;
use crate::routes::router;
use crate::state::AppState;

pub struct AuthJwt;

impl AuthJwt {
    pub async fn run() -> Result<(), anyhow::Error> {
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, CONFIG.port);

        let database_connection_options = ConnectOptions::new(&CONFIG.database_connection_string);
        let db = Database::connect(database_connection_options).await?;

        let app_state = AppState::from(&db);
        let app = router().with_state(app_state);
        let listener = TcpListener::bind(addr).await?;

        axum::serve(listener, app).await?;
        Ok(())
    }
}
