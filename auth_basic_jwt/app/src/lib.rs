use anyhow::Ok;
use axum::{routing::get, Router, };
pub struct Server;


impl Server {
    pub async fn run()-> anyhow::Result<()>{
       let app = Router::new().route("/health", get(|| async { "Service healthy"}));

       let listner =tokio::net::TcpListener::bind("0.0.0.0:3000").await .expect("error starting the application on port 3000");

       axum::serve(listner, app).await.unwrap();

        Ok(()) 
    }
}