use axum::{extract::State, http::StatusCode, Json};
use bcrypt::DEFAULT_COST;
use entity::user_information;
use sea_orm::{EntityTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    error::AppError,
    shared::{ApiResponse, IntoApiResponse, ResponseBody},
    state::AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    email: String,
    password: String,
}

pub async fn register_user(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> ApiResponse<ResponseBody<Value>> {
    let Some(password) = bcrypt::hash(payload.password, DEFAULT_COST).ok() else {
        return Err(AppError::ServerError { message: None });
    };

    // see if the email already exists

    let new_user = user_information::ActiveModel {
        id: Set(Uuid::new_v4()),
        password: Set(password),
        first_name: Set(payload.first_name.trim().to_string().to_lowercase()),
        last_name: Set(payload.last_name.trim().to_string().to_lowercase()),
        email: Set(payload.email.trim().to_lowercase()),
    };

    let res = user_information::Entity::insert(new_user)
        .exec(&app_state.db)
        .await;
    if res.is_err() {
        return Err(AppError::DatabaseError { message: None });
    };

    Ok(ApiResponse::from_parts(
        json!({"message":"Account created successfully"}),
        Some(StatusCode::CREATED),
    ))
}

pub async fn login() {
    unimplemented!()
}

pub async fn refresh_token() {}
