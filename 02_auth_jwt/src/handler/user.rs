use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{verify, DEFAULT_COST};
use entity::{prelude::UserInformation, user_information};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    error::AppError,
    jwt::JwtClaims,
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
pub struct LoginRequest {
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
    if let Ok(Some(_)) = UserInformation::find()
        .filter(user_information::Column::Email.eq(&payload.email))
        .one(&app_state.db)
        .await
    {
        return Err(AppError::ConflictError {
            message: Some("A user with the provided email already exist!".to_string()),
        });
    };

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

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> ApiResponse<ResponseBody<Value>> {
    let Some(Some(user_data)) = UserInformation::find()
        .filter(user_information::Column::Email.eq(&payload.email))
        .one(&app_state.db)
        .await
        .ok()
    else {
        return Err(AppError::ConflictError {
            message: Some("A user with the provided email already exist!".to_string()),
        });
    };

    let Some(is_correct_password) = verify(payload.password, &user_data.password).ok() else {
        return Err(AppError::WrongCredentialsError { message: None });
    };
    if !is_correct_password {
        return Err(AppError::WrongCredentialsError { message: None });
    }
    // sign the token
    let Ok(jwt_token) =
        JwtClaims::new(user_data.email.clone(), user_data.id.to_string()).gen_token()
    else {
        return Err(AppError::ServerError { message: None });
    };

    let response_body = json!({ "jwt_token":jwt_token });

    Ok(ApiResponse::from_parts(response_body, None))
}

pub async fn refresh_token() {}
