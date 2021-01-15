use std::io::Cursor;

use rocket::response::{self, Responder, Response};
use rocket::{http::ContentType, response::ResponseBuilder};
use rocket::{http::Status, request::Request};
use rocket_contrib::json::JsonValue;

#[derive(Debug)]
pub enum Error {
    NotFound,
    InvalidAPIVersion,
}

macro_rules! error_response {
    ($status:expr, $code:expr, $message:expr) => {
        json_value = json!({ "code": $code, "message": $message }).to_string();
        rocket::response::Response::build()
            .status($status)
            .header(ContentType::JSON)
            .sized_body(json_value.len(), Cursor::new(json_value))
            .finalize()
    }
}

impl Error {
    fn response<'f>(status: Status, message: &str, code: u32) -> Response<'f> {
        let json_value = json!({ "code": code, "message": message }).to_string();
        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(json_value.len(), Cursor::new(json_value))
            .finalize()
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'o>, Status> {
        let json_value: String;
        let response = match self {
            InvalidAPIVersion => error_response! { Status::BadRequest, "Invalid API version", 50041) },
            NotFound => error_response! { Status::NotFound, "404: Not Found", 0 },
        };

        Ok(response)
    }
}
