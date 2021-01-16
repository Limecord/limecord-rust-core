use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::responders::error::{Error as ResponseError, GenericError};

#[derive(Debug, Clone)]
pub struct TokenGuard(pub TokenClaims);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub jti: u64,
    pub sub: String,
    pub aud: String,
    pub is_bot: bool,
    exp: usize,
}

const TOKEN_KEY: &'static [u8] = b"epic_nexure";

impl TokenGuard {
    pub fn read_token(token: String) -> Result<Self, GenericError> {
        let decoding_key = &DecodingKey::from_secret(TOKEN_KEY);
        let validation = &Validation::default();
        let token = decode::<TokenClaims>(&token, decoding_key, validation)?;
        Ok(Self(token.claims))
    }

    pub fn generate_token(
        identifier: u64,
        user_id: u64,
        is_bot: bool,
    ) -> Result<String, GenericError> {
        let expires = Utc::now()
            .checked_add_signed(Duration::days(30))
            .expect("utc-datatime-add")
            .timestamp() as usize;

        let encoding_key = &EncodingKey::from_secret(TOKEN_KEY);
        let header = &Header::new(Algorithm::HS256);
        let claims = &TokenClaims {
            jti: identifier,
            sub: user_id.to_string(),
            aud: "limecord".to_string(),
            is_bot: is_bot,
            exp: expires,
        };

        encode(header, claims, encoding_key).map_err(|e| e.into())
    }
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for TokenGuard {
    type Error = ResponseError;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        // validate the header, to make sure it contains the auth header
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            // the header doesn't exist, return failure
            return Outcome::Failure((Status::Unauthorized, ResponseError::Unauthorized));
        }

        let mut is_bot = false;
        let mut token = String::from(keys[0]);
        if token.starts_with("Bot ") {
            is_bot = true;
            token = String::from(&token[4..]);
        }

        match TokenGuard::read_token(token) {
            Ok(token) => {
                if (token.0.is_bot && !is_bot) || (!token.0.is_bot && is_bot) {
                    Outcome::Failure((Status::Unauthorized, ResponseError::Unauthorized))
                } else {
                    Outcome::Success(token)
                }
            }
            Err(_) => Outcome::Failure((Status::Unauthorized, ResponseError::Unauthorized)),
        }
    }
}
