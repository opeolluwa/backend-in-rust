#[tokio::main]
async fn main() {
    app::Server::run()
        .await
        .expect("error starting the application")
}
