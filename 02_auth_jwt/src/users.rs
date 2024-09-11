use std::{ops::DerefMut, sync::Mutex};

use serde::{Deserialize, Serialize};

pub type Users = Vec<User>;

lazy_static::lazy_static! {
  pub  static ref   UsersInformationDatabase: Mutex<Users> = {
    let mut db  = Users::new();
    Mutex::new(db)
  };
}
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}

impl DerefMut for UsersInformationDatabase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}
