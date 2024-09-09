use serde::{Deserialize, Serialize};

pub type Users = Vec<User>;

lazy_static::lazy_static! {
  pub static ref   UsersInformationDatabase: Users = Users::new();
}
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
