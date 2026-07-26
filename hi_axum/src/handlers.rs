use axum::{
    Json,
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
};

use crate::adapters::{Pagination, User};

pub async fn health_check() -> impl IntoResponse {
    "healthy"
}

// path variables workspace/:workspace_id/user/:id
// workspace/
// workspace/:id
// workspace/:id/user/:userId
pub async fn extract_path(Path(user_identifier): Path<String>) -> impl IntoResponse {
    user_identifier
}

pub async fn extract_multiple_path(
    Path((workspace_identifier, user_identifier)): Path<(String, String)>,
) -> impl IntoResponse {
    format!("{user_identifier}\n{workspace_identifier}")
}

// /search?username=&password=password
pub async fn extract_query(Query(user): Query<User>) -> impl IntoResponse {
    format!("username: {}\npassword:{}", user.username, user.password)
}

pub async fn extract_json(Json(user): Json<User>) -> impl IntoResponse {
    format!("username: {}\npassword:{}", user.username, user.password)
}

// profile/:id?pageSize=5&currentPage=3
pub async fn extract_multiple_types(
    Query(pagination): Query<Pagination>,
    Path(user_identifier): Path<String>,
    Json(user): Json<User>,
) -> impl IntoResponse {
    dbg!("{}", user);
    dbg!("{}", pagination);
    dbg!("{}", user_identifier);

    "response"
}

pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "the resource you are looking for does not not exist or has been removed",
    )
}
