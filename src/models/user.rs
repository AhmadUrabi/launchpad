use diesel::{connection::LoadConnection, prelude::*};

use serde::{Deserialize, Serialize};

use crate::traits::{Parameter, CRUD};

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

use diesel::mysql::Mysql;

impl CRUD for User {
    type Backend = Mysql;
    fn all<C>(conn: &mut C) -> Result<Vec<Self>, String>
    where
        C: LoadConnection<Backend = Self::Backend>,
    {
        use crate::schema::users::dsl::*;

        let result = users.select(User::as_select()).load::<User>(conn);

        match result {
            Ok(u) => Ok(u),
            Err(e) => Err(e.to_string()),
        }
    }

    fn find_first<C>(conn: &mut C, id: i64) -> Result<Self, String>
    where
        C: LoadConnection<Backend = Self::Backend>,
    {
        use crate::schema::users::dsl::*;

        let result = users.find(id).first::<User>(conn);

        match result {
            Ok(u) => Ok(u),
            Err(e) => Err(e.to_string()),
        }
    }

    fn delete<C>(&self, conn: &mut C) -> Result<(), String>
    where
        C: LoadConnection<Backend = Self::Backend>,
    {
        use crate::schema::users::dsl::*;
        let result = diesel::delete(users.find(self.id)).execute(conn);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn update<C>(&self, conn: &mut C, params: Vec<Parameter>) -> Result<(), String>
    where
        C: LoadConnection<Backend = Self::Backend>,
    {
        Ok(())
    }
}
