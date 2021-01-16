#![feature(backtrace)]

use responders::error::Error;
use rocket::{http::Status, response, Catcher, Request};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod controllers;
mod guards;
mod responders;

// fn default_catcher<'r>(req: &'r Request) -> response::Result<'r> {}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api", controllers::get_routes())
    // .register(catchers![Catcher::new(None, default_catcher)])
}
