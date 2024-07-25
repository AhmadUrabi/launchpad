#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
    password: String,
}
