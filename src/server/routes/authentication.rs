use rocket::{serde::json::Json, State};

use crate::{models::user::User, server::state::ServerState};

#[derive(serde::Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<login>")]
pub async fn login(state: &State<ServerState>, login: Json<LoginParams>) -> String {
    let db = &state.db.clone();
    let user = User::login(login, db);
    if let Ok(user) = user {
        return format!("Welcome, {}!", user.name);
    }
    "Login failed".to_owned()
}
