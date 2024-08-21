use rocket::serde::json::Json;

use crate::models::user::User;

pub trait Model: Sized {
    const TABLE_NAME: &'static str;
    fn all() -> Result<Vec<Self>, String>;
    fn find(query_id: u64) -> Result<Self, String>;
    fn create(data: Json<rocket::serde::json::Value>) -> Result<String, String>;
    // fn update(&self, id: u64, params: Vec<Parameter>) -> Result<Self, String>;
    // fn delete(&self, id: u64) -> Result<(), String>;
}

pub trait Controller: Model {
    fn index(&self) -> Result<(), String>;
    fn show(&self, id: u64) -> Result<(), String>;
    fn create(&self, params: Vec<Parameter>) -> Result<(), String>;
    fn update(&self, id: u64, params: Vec<Parameter>) -> Result<(), String>;
    fn delete(&self, id: u64) -> Result<(), String>;
}

pub trait Permission: Model {
    fn read(resource: Option<Self>, user: User) -> bool;
    fn write(resource: Option<Self>, user: User) -> bool;
    fn update(resource: Option<Self>, user: User) -> bool;
    fn delete(resource: Option<Self>, user: User) -> bool;
    fn custom(resource: Option<Self>, user: User, name: &'static str) -> bool;
}

pub struct Parameter {
    pub name: &'static str,
    pub value: &'static str,
}
