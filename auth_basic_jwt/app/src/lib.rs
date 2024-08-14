use std::time::Duration;

use anyhow::Ok;
use axum::{routing::get, Router};
use config::Config;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ ConnectOptions, Database};

mod config;
mod errors;
mod handlers;
mod router;
pub struct Server;

impl Server {
    pub async fn run() -> anyhow::Result<()> {
        let database_url = Config::parse().database_url;

        let mut opt = ConnectOptions::new(&database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        let db = Database::connect(opt).await?;

        let app = Router::new().route("/health", get(|| async { "Service healthy" }));

        // auto migrate the database

        let connection = sea_orm::Database::connect(&database_url).await?;
        Migrator::up(&connection, None).await?;

        let listner = tokio::net::TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("error starting the application on port 3000");

        println!(
            "\nserver listening on http://0.0.0.0:3000\nhealth check at http://0.0.0.0:3000/health"
        );
        axum::serve(listner, app).await.unwrap();

        Ok(())
    }
}
