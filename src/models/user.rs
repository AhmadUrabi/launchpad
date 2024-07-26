use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::{db::DB, server::routes::authentication::LoginParams};

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(name: String, email: String, password: String) -> Self {
        User {
            name,
            email,
            password,
        }
    }

    pub fn login(credentials: Json<LoginParams>, db: &DB) -> Result<Self, String> {
        // TODO: Implement this method
        Ok(User::new(
            "name".to_owned(),
            "email".to_owned(),
            "password".to_owned(),
        ))
    }
}
