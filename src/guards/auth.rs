use rocket::{
    http::hyper::StatusCode,
    request::{FromRequest, Outcome},
    Request,
};

use super::token::TokenGuard;
use crate::responders::error::Error as ResponseError;

pub struct AuthGuard();

impl AuthGuard {}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for AuthGuard {
    type Error = ResponseError;

    async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let token = try_outcome!(request.guard::<TokenGuard>().await);
        println!("token: {:?}", token);

        Outcome::Success(Self())
    }
}
