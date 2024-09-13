use super::handler::hello_world_handler;
use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/", get(hello_world_handler))
}
