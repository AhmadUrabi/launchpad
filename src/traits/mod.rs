use diesel::connection::LoadConnection;

pub struct Parameter {
    pub key: String,
    pub value: Option<String>,
}

pub trait CRUD: Sized {
    type Backend;
    fn all<C>(conn: &mut C) -> Result<Vec<Self>, String>
    where
        C: LoadConnection<Backend = Self::Backend>;

    fn find_first<C>(conn: &mut C, id: i64) -> Result<Self, String>
    where
        C: LoadConnection<Backend = Self::Backend>;

    fn delete<C>(&self, conn: &mut C) -> Result<(), String>
    where
        C: LoadConnection<Backend = Self::Backend>;

    fn update<C>(&self, conn: &mut C, params: Vec<Parameter>) -> Result<(), String>
    where
        C: LoadConnection<Backend = Self::Backend>;
}
