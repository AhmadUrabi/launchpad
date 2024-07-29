use diesel::prelude::*;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rocket::{serde::json::Json, State};

use rocket::http::{Cookie, CookieJar};

use crate::{
    models::user::User,
    server::{
        controllers::auth_controller::Claims,
        responses::{ApiOk, Response},
        state::ServerState,
    },
};

#[derive(serde::Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<login>")]
pub async fn login(
    state: &State<ServerState>,
    login: Json<LoginParams>,
    cookies: &CookieJar<'_>,
) -> Json<Response> {
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
            // Serialize user to text
            if user.password == login.0.password {
                let claims = Claims::generate_from_user(user);
                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret("secret".as_ref()),
                )
                .unwrap();
                let cookie = Cookie::build(("Authorization", format!("Bearer {}", token.clone())))
                    .path("/")
                    .secure(true)
                    .http_only(true);
                cookies.add(cookie);
                return Json(Response {
                    status: "success".to_owned(),
                    message: None,
                });
            } else {
                return Json(Response {
                    status: "error".to_owned(),
                    message: Some("Invalid password".to_owned()),
                });
            }
        }
        Err(_) => {
            return Json(Response {
                status: "error".to_owned(),
                message: Some("User not found".to_owned()),
            });
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
    let connection = std::env::var("DATABASE_URL").unwrap();
    let mut connection = MysqlConnection::establish(&connection).unwrap();
    // TODO: Check id error
    let user = User {
        id: 0,
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
