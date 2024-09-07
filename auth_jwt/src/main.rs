#[tokio::main]

async fn main() -> Result<(), std::io::Error> {
    pkg::app::AuthJwt::run().await
}
