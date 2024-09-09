use axum::{routing::get, Router};

use crate::handler::{root::base, root::health_check_handler};

pub fn router() -> Router {
    Router::new()
        .route("/", get(base))
        .route("/health", get(health_check_handler))
}
