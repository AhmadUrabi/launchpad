use rocket::serde::json::Json;

use crate::db::DB;

#[derive(serde::Deserialize)]
struct LoginParams {
    username: String,
    password: String,
}

#[post("/login", data = "<login>")]
pub async fn login(login: Json<LoginParams>) -> String {
    format!("Hello, {}!", login.username)
}
