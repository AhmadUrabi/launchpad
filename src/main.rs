#[macro_use]
extern crate rocket;

pub(crate) mod db;
pub(crate) mod models;
pub(crate) mod schema;
pub(crate) mod server;

#[rocket::main]
async fn main() {
    dotenv::dotenv().ok();
    let server = server::Server::init();
    match server.launch().await {
        Err(e) => println!("Error starting server: {}", e),
        _ => (),
    }
}
