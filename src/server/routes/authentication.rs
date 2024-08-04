use diesel::prelude::*;

use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::{serde::json::Json, State};

use rocket::http::{Cookie, CookieJar};

use crate::traits::CRUD;
use crate::{
    models::user::User,
    server::{controllers::auth_controller::Claims, responses::Response, state::ServerState},
};

#[derive(serde::Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<login>")]
pub async fn login(
    state: &State<ServerState<MysqlConnection>>,
    login: Json<LoginParams>,
    cookies: &CookieJar<'_>,
) -> Json<Response> {
    let connection = &mut state.pool.get().unwrap();
    let user_list = User::all(connection).unwrap();

    let user = user_list
        .iter()
        .find(|u| u.email == login.0.email && u.password == login.0.password);

    if user.is_none() {
        return Json(Response {
            status: "Failed".to_owned(),
            message: Some("Invalid email or password".to_owned()),
        });
    }

    let user = user.unwrap();

    let claims = Claims::generate_from_user(user);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_bytes()),
    )
    .unwrap();

    cookies.add(Cookie::new("Authorization", token.clone()));

    return Json(Response {
        status: "Success".to_owned(),
        message: Some(token),
    });
}

#[derive(serde::Deserialize)]
pub struct RegisterParams {
    pub name: String,
    pub email: String,
    pub password: String,
    // pub role: String,
}

#[post("/register", data = "<register>")]
pub async fn register(
    state: &State<ServerState<MysqlConnection>>,
    register: Json<RegisterParams>,
) -> String {
    use crate::schema::users::dsl::*;

    let connection = &mut state.pool.get().unwrap();

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
        .execute(connection)
        .unwrap();

    return "Register successful".to_owned();
}
