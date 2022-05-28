#[macro_use]
extern crate rocket;

mod constants;
mod paste_id;
mod todo;

use constants::ID_SIZE;
use paste_id::PasteId;
use rocket::data::ToByteUnit;
use rocket::form::Form;
use rocket::http::uri::Absolute;
use rocket::response::Redirect;
use rocket::tokio::fs::{self, File};
use rocket::{Data, Request};
use rocket_dyn_templates::{context, Template};
use std::io::prelude::*;

#[derive(FromForm)]
struct Paste<'r> {
    data: &'r str,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[get("/show/<id>")]
async fn show_paste(id: PasteId<'_>) -> Template {
    let mut content = String::new();
    let mut error = false;

    match fs::read_to_string(id.file_path()).await {
        Ok(p) => content.push_str(&p),
        Err(_) => error = true,
    }

    Template::render(
        "show",
        context! {
            paste: content,
            error: error,
        },
    )
}

#[post("/create", data = "<paste>")]
async fn create_paste(paste: Form<Paste<'_>>) -> Redirect {
    let id = PasteId::new(ID_SIZE);
    let paste_bytes = paste.data.as_bytes();

    match std::fs::File::create(id.file_path()) {
        Ok(mut file) => match file.write_all(paste_bytes) {
            Ok(_) => return Redirect::to(uri!("/", show_paste(id))),
            Err(_) => return Redirect::to(uri!("/", index())),
        },
        Err(_) => return Redirect::to(uri!("/", index())),
    }
}

#[post("/v1/create", data = "<paste>")]
async fn create_api(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_SIZE);
    let host: Absolute<'static> = uri!("http://localhost:8000");
    paste
        .open(128.kibibytes())
        .into_file(id.file_path())
        .await?;

    Ok(uri!(host, read_api(id)).to_string())
}

#[get("/v1/read/<id>")]
async fn read_api(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[put("/v1/update/<id>", data = "<paste>")]
async fn update_api(id: PasteId<'_>, paste: Data<'_>) -> std::io::Result<String> {
    let host: Absolute<'static> = uri!("http://localhost:8000");
    paste
        .open(128.kibibytes())
        .into_file(id.file_path())
        .await?;

    Ok(uri!(host, read_api(id)).to_string())
}

#[delete("/v1/delete/<id>")]
async fn delete_api(id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, show_paste, create_paste])
        .mount(
            "/api",
            routes![create_api, read_api, update_api, delete_api],
        )
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}
