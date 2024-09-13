use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        protected::some_protected_resource,
        root::{base, health_check_handler},
        user::{login, register_user},
    },
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(base))
        .route("/health", get(health_check_handler))
        .nest("/users", users_route())
        .nest("/api", protected_routes())
}

fn users_route() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login))
}

fn protected_routes() -> Router<AppState> {
    Router::new().route("/protected", get(some_protected_resource))
}
