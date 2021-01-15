use rocket::http::Status;
use crate::api_version_check;

#[post("/<version>/science")]
pub fn science(version: String) -> Status {
    api_version_check!(version, 6);
    Status::NoContent
}