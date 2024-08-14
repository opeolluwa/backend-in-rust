#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    app::Server::run()
        .await
        .expect("error starting the application")
}
