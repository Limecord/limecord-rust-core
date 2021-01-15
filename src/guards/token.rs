use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::responders::error::Error;

pub struct TokenGuard(pub TokenClaims);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub jti: i32,
    pub sub: String,
    pub aud: String,
    pub is_bot: bool,
    exp: usize,
}

impl TokenGuard {
    pub fn read_token(token: String) -> Result<Self, Error> {   
        let decoding_key = &DecodingKey::from_secret(secret.as_bytes());
        let validation = &Validation::default();
        match decode::<TokenClaims>(&token, decoding_key, validation) {
            Ok(token) => Ok(Self(token.claims)),
            Err(_) => Err(Error::Unauthorized),
        }
    }
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for TokenGuard {
    type Error = ();

    async fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // validate the header, to make sure it contains the auth header
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            // the header doesn't exist, return failure
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let mut is_bot = false;
        let token = String::from(keys[0]);
        if token.starts_with("Bot ") {
            is_bot = true;
            token = String::from(&token[4..]);
        }

        match TokenGuard::read_token(token) {
            Ok(token) => Outcome::Success(token),
            Err(e) => Outcome::
        }

    }
}