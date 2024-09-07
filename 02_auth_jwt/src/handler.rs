pub async fn root() -> &'static str {
    "The Application is running on https://localhost:3000"
}

pub async fn health_check_handler() -> &'static str {
    "Auth JWT service is healthy"
}

pub async fn register() {
    unimplemented!()
}

pub async fn login() {
    unimplemented!()
}

pub async fn refresh_token() {}
