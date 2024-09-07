use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

impl User {
    pub fn new(email: &str, raw_password: &str, first_name: &str, last_name: &str) -> Self {
        Self {
            email: email.trim().to_string(),
            password_hash: todo!(),
            first_name: first_name.trim().to_string(),
            last_name: last_name.trim().to_string(),
        }
    }

    pub fn save(&self) {}
}

pub type Users = Vec<User>;
