use axum::{routing::get, Router};
use super::handler::hello_world_handler;

pub fn router( ) -> Router {
    Router::new().route("/", get(hello_world_handler))
}