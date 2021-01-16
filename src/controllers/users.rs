use crate::{guards::auth::AuthGuard, responders::error::Error};

use super::api_version_check;

#[get("/<version>/users/<user>")]
pub fn user(version: String, user: String, _auth: AuthGuard) -> Result<(), Error> {
    api_version_check(version, 6)?;

    println!("User: {:?}", user);

    Ok(())
}
