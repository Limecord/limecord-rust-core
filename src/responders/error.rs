use std::io::Cursor;

use rocket::http::ContentType;
use rocket::response::{Responder, Response};
use rocket::{http::Status, request::Request};

// See reference: https://discord.com/developers/docs/topics/opcodes-and-status-codes
#[derive(Debug, Clone)]
pub enum Error {
    // these errors have code 0
    NotFound,
    Unauthorized,

    // these errors have different codes and statuses
    // 50041: Invalid API version provided
    InvalidAPIVersion,
}

impl Error {
    fn get_error_data(&self) -> (Status, &str, u32) {
        match self {
            Self::InvalidAPIVersion => (Status::BadRequest, "Invalid API version", 50041),
            Self::Unauthorized => (Status::Unauthorized, "404: Unauthorized", 0),
            Self::NotFound => (Status::NotFound, "404: Not Found", 0),
        }
    }

    #[inline]
    fn to_response<'o>(&self) -> Response<'o> {
        let data = self.get_error_data();
        let json_str = json!({ "code": data.2, "message": data.1 }).to_string();
        Response::build()
            .status(data.0)
            .header(ContentType::JSON)
            .sized_body(json_str.len(), Cursor::new(json_str))
            .finalize()
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'o>, Status> {
        Ok(self.to_response())
    }
}
