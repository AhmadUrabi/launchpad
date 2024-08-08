use diesel::prelude::*;

use serde::{Deserialize, Serialize};

use crate::traits::Model;

#[derive(Insertable, Queryable, Selectable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub role: String,
    pub password: String,
}

impl Model for User {
    const TABLE_NAME: &'static str = "users";
    fn all() -> Result<Vec<Self>, String> {
        use crate::schema::users::dsl::*;
        let mut conn = crate::db::get_conn();
        let result = users.load::<User>(&mut conn);
        match result {
            Ok(_users) => Ok(_users),
            Err(e) => Err(e.to_string()),
        }
    }
    fn find(query_id: u64) -> Result<Self, String> {
        use crate::schema::users::dsl::*;
        let mut conn = crate::db::get_conn();
        let result = users.find(query_id).first::<User>(&mut conn);
        match result {
            Ok(user) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }
    fn create(data: Self) -> Result<Self, String> {
        use crate::schema::users::dsl::*;
        let mut conn = crate::db::get_conn();
        let result = diesel::insert_into(users).values(&data).execute(&mut conn);
        match result {
            Ok(_) => Ok(data),
            Err(e) => Err(e.to_string()),
        }
    }
    // fn update(&self, id: u64, params: Vec<Parameter>) -> Result<Self, String> {
    //     Ok(())
    // }
    // fn delete(&self, id: u64) -> Result<(), String> {
    //     Ok(())
    // }
}
