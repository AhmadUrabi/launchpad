use rocket::{Ignite, Rocket};
use state::ServerState;

pub(crate) mod catchers;
pub(crate) mod fairings;
pub(crate) mod guards;
pub(crate) mod state;

pub(crate) struct Server {
    server: rocket::Rocket<rocket::Build>,
}

impl Server {
    pub fn init() -> Self {
        let routes = routes![];
        let state = ServerState { db: "".to_owned() };
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
