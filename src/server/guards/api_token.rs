use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub struct ApiToken(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiToken {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // TODO: Replace this with a proper authentication mechanism
        let api_token = req.headers().get_one("X-API-TOKEN");
        match api_token {
            Some(token) => Outcome::Success(ApiToken(token.to_string())),
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
