pub mod authentication;
pub mod users;

pub fn routes() -> Vec<rocket::Route> {
    routes![
        authentication::login,
        authentication::register,
        users::get_users
    ]
}
