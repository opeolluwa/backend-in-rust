use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::root::{base, health_check_handler},
    handler::user::register_user,
    state::AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(base))
        .route("/health", get(health_check_handler))
        .nest("/users", users_route())
}

fn users_route() -> Router<AppState> {
    Router::new().route("/register", post(register_user))
}
