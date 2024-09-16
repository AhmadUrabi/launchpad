use jsonwebtoken::{TokenData, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::{models::user::User, server::controllers::auth_controller::Claims, traits::Model};

pub struct ApiToken(pub User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiToken {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // TODO: Replace this with a proper authentication mechanism
        let api_token = req.cookies().get("Authorization");

        match api_token {
            Some(token) => {
                let decoded_key = jsonwebtoken::decode(
                    token.value(),
                    &jsonwebtoken::DecodingKey::from_secret("secret".as_bytes()),
                    &Validation::new(jsonwebtoken::Algorithm::HS256),
                );

                if decoded_key.is_err() {
                    return Outcome::Error((Status::Unauthorized, ()));
                }

                let decoded_key: TokenData<Claims> = decoded_key.unwrap();

                let user = User::find(decoded_key.claims.id as u64);

                if user.is_err() {
                    return Outcome::Error((Status::Unauthorized, ()));
                }

                Outcome::Success(ApiToken(user.unwrap()))
            }
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
