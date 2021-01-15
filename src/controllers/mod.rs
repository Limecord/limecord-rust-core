use crate::responders::error::Error;
use std::str::FromStr;

pub mod tracking;

#[inline]
fn api_version_check<'t>(input_version: String, min_version: u8) -> Result<(), Error> {
    if input_version.len() < 1 {
        return Err(Error::NotFound);
    }

    let version = u8::from_str(&input_version[1..]).map_err(|_| Error::NotFound)?;
    println!("Version: {:?}", version);

    if version >= min_version {
        Ok(())
    } else {
        return Err(Error::InvalidAPIVersion);
    }
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![tracking::science]
}
