#[macro_use]
extern crate rocket;

mod paste_id;

use paste_id::PasteId;
use rocket::{Rocket, Build};
use rocket::tokio::fs::File;

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

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, retrieve])
}
