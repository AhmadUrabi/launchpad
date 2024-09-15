pub mod authentication;
pub mod users;

pub fn routes() -> Vec<rocket::Route> {
    let user_routes = users::routes();
    let auth_routes = authentication::routes();

    let mut routes = Vec::new();
    routes.extend(user_routes);
    routes.extend(auth_routes);

    routes
}
