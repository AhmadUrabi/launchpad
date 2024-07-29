use rocket::{Ignite, Rocket};
use state::ServerState;

use crate::db::DB;

pub(crate) mod catchers;
pub(crate) mod controllers;
pub(crate) mod fairings;
pub(crate) mod guards;
pub(crate) mod responses;
pub(crate) mod routes;
pub(crate) mod state;

pub(crate) struct Server {
    server: rocket::Rocket<rocket::Build>,
}

impl Server {
    pub fn init() -> Self {
        let routes = routes![
            routes::authentication::login,
            routes::authentication::register
        ];
        let state = ServerState { db: DB::init() };
        let rocket = rocket::build()
            .attach(fairings::cors::CORS)
            .register("/", catchers![catchers::not_found])
            .manage(state)
            .mount("/api", routes);
        Self { server: rocket }
    }

    pub async fn launch(self) -> Result<Rocket<Ignite>, rocket::Error> {
        self.server.launch().await
    }
}
