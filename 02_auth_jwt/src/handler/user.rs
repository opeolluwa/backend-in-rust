use std::any::Any;

use axum::Json;
use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};

use crate::{
    users::{User, UsersInformationDatabase},
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

pub async fn register(Json(payload): Json<RegisterRequest>) -> Result<&'static str, &'static str> {
    let Ok(password) = bcrypt::hash(payload.password, DEFAULT_COST) else {
        return Err("An unexpected error happened");
    };

    let user = User {
        email: payload.email,
        password: password,
        first_name: payload.first_name,
        last_name: payload.last_name,
    };
    // store the user in the database
    let mut db =  UsersInformationDatabase.lock().unwrap();
    // *db = db.push(user);
    Ok("User successfully signed up")
}

pub async fn login() {
    unimplemented!()
}

pub async fn refresh_token() {}
