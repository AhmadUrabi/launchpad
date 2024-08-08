use std::sync::Mutex;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    MysqlConnection,
};

pub static POOL: Mutex<Option<Pool<ConnectionManager<MysqlConnection>>>> = Mutex::new(None);

pub fn create_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(std::env::var("DATABASE_URL").unwrap());
    let pool = Pool::builder().build(manager).unwrap();
    pool
}

pub fn get_conn() -> PooledConnection<ConnectionManager<MysqlConnection>> {
    POOL.lock().unwrap().as_ref().unwrap().get().unwrap()
}
