pub async fn base() -> &'static str {
    "The Application is running on https://localhost:3000"
}

pub async fn health_check_handler() -> &'static str {
    "Auth JWT service is healthy"
}
