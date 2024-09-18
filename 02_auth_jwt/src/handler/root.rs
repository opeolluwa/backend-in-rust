pub async fn base() -> &'static str {
    "The application is running on https://localhost:3000"
}

// health check
pub async fn health_check_handler() -> &'static str {
    "Auth JWT service is healthy"
}
