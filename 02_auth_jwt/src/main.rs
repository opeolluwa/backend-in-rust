use migration::{Migrator, MigratorTrait};
use pkg::config::CONFIG;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connection = sea_orm::Database::connect(&CONFIG.database_connection_string).await?;

    Migrator::up(&connection, None).await?;

    pkg::app::AuthJwt::run().await
}
