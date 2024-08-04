use diesel::MysqlConnection;
use rocket::{Ignite, Rocket};
use routes::authentication;
use state::ServerState;

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

        let database_url = std::env::var("DATABASE_URL").unwrap();
        let state: ServerState<MysqlConnection> = ServerState::new(database_url.as_str()).unwrap();

        let rocket = rocket::build()
            .attach(fairings::cors::CORS)
            .register("/", catchers)
            .manage(state)
            .mount("/api", routes);

        Self { server: rocket }
    }

    pub async fn launch(self) -> Result<Rocket<Ignite>, rocket::Error> {
        self.server.launch().await
    }
}
