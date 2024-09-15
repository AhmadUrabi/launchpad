use rocket::{Ignite, Rocket};

use crate::db;

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
        let routes = routes::routes();
        let catchers = catchers::catchers();

        *db::POOL.lock().unwrap() = Some(db::create_pool());

        let rocket = rocket::build()
            .attach(fairings::cors::CORS)
            .register("/", catchers)
            .mount("/api", routes);

        Self { server: rocket }
    }

    pub async fn launch(self) -> Result<Rocket<Ignite>, rocket::Error> {
        self.server.launch().await
    }
}
