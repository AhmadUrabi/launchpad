use crate::models::user::User;
use crate::server::guards::api_token::ApiToken;
use crate::traits::Model;
use rocket::serde::json::Json;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_users, get_user_by_id, create_user]
}

#[get("/users")]
pub async fn get_users(_token: ApiToken) -> Result<Json<Vec<User>>, String> {
    match User::all() {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(e.to_string()),
    }
}

#[get("/users/<id>")]
pub async fn get_user_by_id(id: u64, _token: ApiToken) -> Result<Json<User>, String> {
    match User::find(id) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e.to_string()),
    }
}

#[post("/users", data = "<user>")]
pub async fn create_user(
    user: Json<rocket::serde::json::Value>,
    _token: ApiToken,
) -> Result<Json<String>, String> {
    println!("{:?}", user);

    match User::create(user) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e.to_string()),
    }
}
