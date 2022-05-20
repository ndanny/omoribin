#[macro_use]
extern crate rocket;

mod paste_id;

use paste_id::PasteId;
use rocket::{Build, Data, Rocket};
use rocket::data::ToByteUnit;
use rocket::http::uri::Absolute;
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

/// Todo
/// - Store id and host in a config file
#[post("/", data = "<paste>")]
async fn save(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(16);
    let host: Absolute<'static> = uri!("http://localhost:8000");
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;

    Ok(uri!(host, retrieve(id)).to_string())
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, retrieve, save])
}
