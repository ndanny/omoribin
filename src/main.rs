use rocket::{Rocket, Build};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
    
        POST /

            accepts raw data from the request body and responds
            with a url of the paste

        GET /<id>

            retrieves content for paste <id>
    "
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index])
}
