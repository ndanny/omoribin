#[macro_use]
extern crate rocket;

mod constants;
mod paste_id;

use constants::ID_SIZE;
use paste_id::PasteId;
use rocket::{Build, Data, Rocket};
use rocket::data::ToByteUnit;
use rocket::http::uri::Absolute;
use rocket::tokio::fs::{File, self};

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

#[post("/new", data = "<paste>")]
async fn save(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_SIZE);
    let host: Absolute<'static> = uri!("http://localhost:8000");
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;

    Ok(uri!(host, retrieve(id)).to_string())
}

#[delete("/delete/<id>")]
async fn delete(id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, retrieve, save, delete])
}
