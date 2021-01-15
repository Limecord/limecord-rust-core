#[macro_use] extern crate rocket;
mod controllers;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/api", controllers::get_routes())
}