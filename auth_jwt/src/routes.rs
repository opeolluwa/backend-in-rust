use axum::{routing::get, Router};

use crate::handler::{health_check_handler, root};

pub fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check_handler))
}
