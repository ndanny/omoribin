#[macro_use]
extern crate rocket;

mod constants;
mod paste_id;
mod todo;

use constants::ID_SIZE;
use paste_id::PasteId;
use rocket::data::ToByteUnit;
use rocket::http::uri::Absolute;
use rocket::tokio::fs::{self, File};
use rocket::{Build, Data, Rocket};

#[get("/")]
fn index() -> &'static str {
    ""
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[post("/new", data = "<paste>")]
async fn save(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_SIZE);
    let host: Absolute<'static> = uri!("http://localhost:8000");
    paste
        .open(128.kibibytes())
        .into_file(id.file_path())
        .await?;

    Ok(uri!(host, retrieve(id)).to_string())
}

#[delete("/delete/<id>")]
async fn delete(id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}

#[put("/replace/<id>", data = "<paste>")]
async fn replace(id: PasteId<'_>, paste: Data<'_>) -> std::io::Result<String> {
    let host: Absolute<'static> = uri!("http://localhost:8000");
    paste
        .open(128.kibibytes())
        .into_file(id.file_path())
        .await?;

    Ok(uri!(host, retrieve(id)).to_string())
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, retrieve, save, delete, replace])
}
