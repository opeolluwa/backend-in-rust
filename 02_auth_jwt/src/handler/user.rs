use axum::Json;
use serde::{Deserialize, Serialize};

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


pub async fn register(Json(payload): Json<RegisterRequest>) {
    unimplemented!()
}

pub async fn login() {
    unimplemented!()
}

pub async fn refresh_token() {}
