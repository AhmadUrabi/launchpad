// use std::sync::{Arc, Mutex};

use crate::db::DB;

pub(crate) struct ServerState {
    pub(crate) db: DB,
}
