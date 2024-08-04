use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::r2d2::R2D2Connection;

pub(crate) struct ServerState<C: R2D2Connection + 'static> {
    pub pool: Pool<ConnectionManager<C>>,
}

impl<C> ServerState<C>
where
    C: R2D2Connection + 'static,
{
    pub fn new(database_url: &str) -> Result<Self, String> {
        let manager = ConnectionManager::<C>::new(database_url);
        let pool = Pool::builder().build(manager).unwrap();
        Ok(Self { pool })
    }
}

// use rocket::fairing::AdHoc;
// use rocket::Build;
// use rocket::Rocket;
// use rocket_db_pools::{Database, Poolable};

// #[derive(Database)]
// #[database("my_db")]
// pub struct ServerState<C: Poolable>(pub rocket_db_pools::Pool<C>);
