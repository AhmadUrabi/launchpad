use crate::models::user::User;
use crate::server::guards::api_token::ApiToken;
use crate::server::state::ServerState;
use crate::traits::CRUD;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket::State;

#[get("/users")]
pub async fn get_users(
    _token: ApiToken,
    state: &State<ServerState<MysqlConnection>>,
) -> Result<Json<Vec<User>>, String> {
    println!("{}", _token.0);
    let connection = &mut state.pool.get().unwrap();

    match User::all(connection) {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(e.to_string()),
    }
}
