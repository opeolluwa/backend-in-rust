use std::net::{Ipv4Addr, SocketAddrV4};

use std::io::Error;
use tokio::net::TcpListener;

use crate::routes::router;

pub struct AuthJwt;

impl AuthJwt {
    pub async fn run() -> Result<(), Error> {
        let ip = Ipv4Addr::new(0, 0, 0, 0);
        let port = crate::config::CONFIG.port;
        let addr = SocketAddrV4::new(ip, port);

        let app = router();
        let listener = TcpListener::bind(addr).await?;

        axum::serve(listener, app).await
    }
}
