use crate::responders::error::Error;

use super::api_version_check;
use rocket::http::Status;

#[post("/<version>/science")]
pub fn science<'a>(version: String) -> Result<Status, Error> {
    api_version_check(version, 6)?;
    Ok(Status::NoContent)
}
