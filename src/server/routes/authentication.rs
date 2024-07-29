use diesel::prelude::*;

use rocket::{serde::json::Json, State};

use crate::{models::user::User, server::state::ServerState};

#[derive(serde::Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<login>")]
pub async fn login(state: &State<ServerState>, login: Json<LoginParams>) -> String {
    use crate::schema::users::dsl::*;
    // let db = &state.db.clone();
    //
    let connection = std::env::var("DATABASE_URL").unwrap();
    let mut connection = MysqlConnection::establish(&connection).unwrap();
    let user = users
        .filter(email.eq(login.0.email))
        .select(User::as_select())
        .first::<User>(&mut connection);

    match user {
        Ok(user) => {
            if user.password == login.0.password {
                return "Login successful".to_owned();
            } else {
                return "Login failed".to_owned();
            }
        }
        Err(_) => {
            return "Login failed".to_owned();
        }
    }
}

#[derive(serde::Deserialize)]
pub struct RegisterParams {
    pub name: String,
    pub email: String,
    pub password: String,
    // pub role: String,
}

#[post("/register", data = "<register>")]
pub async fn register(state: &State<ServerState>, register: Json<RegisterParams>) -> String {
    use crate::schema::users::dsl::*;
    // let db = &state.db.clone();
    //
    let connection = std::env::var("DATABASE_URL").unwrap();
    let mut connection = MysqlConnection::establish(&connection).unwrap();
    let user = User {
        name: register.0.name.clone(),
        email: register.0.email.clone(),
        password: register.0.password.clone(),
        role: "user".to_owned(),
    };

    diesel::insert_into(users)
        .values(&user)
        .execute(&mut connection)
        .unwrap();

    return "Register successful".to_owned();
}
