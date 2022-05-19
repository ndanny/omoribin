use rocket::{Rocket, Build};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
}
