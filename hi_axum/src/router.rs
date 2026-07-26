use crate::handlers::{
    extract_json, extract_multiple_path, extract_multiple_types, extract_path, extract_query,
    handle_404, health_check,
};
use axum::{
    Router,
    routing::{get, post, put},
};

pub fn routes() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/user/{user_identifier}", get(extract_path))
        .route(
            "/workspace/{workspace_identifie}/user/{user_identifier}",
            get(extract_multiple_path),
        )
        .route("/login", post(extract_query))
        .route("/profile", put(extract_json))
        .route("/profile/{id}", post(extract_multiple_types))
        .fallback(handle_404)
}
