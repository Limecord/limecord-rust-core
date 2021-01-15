#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

mod controllers;
mod guards;
mod responders;

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api", controllers::get_routes())
}
