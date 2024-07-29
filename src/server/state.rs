use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

use diesel::MysqlConnection;

pub(crate) struct ServerState {
    pub pool: Pool<ConnectionManager<MysqlConnection>>,
}

impl ServerState {
    pub fn new() -> Self {
        let pool = Self::get_connection_pool();
        Self { pool }
    }

    fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
        let url = std::env::var("DATABASE_URL").unwrap();
        let manager = ConnectionManager::<MysqlConnection>::new(url);
        // Refer to the `r2d2` documentation for more methods to use
        // when building a connection pool
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    }
}
