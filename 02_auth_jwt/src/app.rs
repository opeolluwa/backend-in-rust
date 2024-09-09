use std::net::{Ipv4Addr, SocketAddrV4};

use std::io::Error;
use tokio::net::TcpListener;

use crate::config::CONFIG;
use crate::routes::router;

pub struct AuthJwt;

impl AuthJwt {
    pub async fn run() -> Result<(), Error> {
        let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, CONFIG.port);

        let app = router();
        let listener = TcpListener::bind(addr).await?;

        axum::serve(listener, app).await
    }
}
