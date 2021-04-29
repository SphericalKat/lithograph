#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rust_embed;

use std::{ffi::OsStr, io::Cursor, path::PathBuf};

use askama::Template;
use chrono::{Datelike, Local};
use rocket::{
    http::{ContentType, Status},
    response,
};

#[derive(RustEmbed)]
#[folder = "public/"]
struct Static;

#[derive(Template)]
#[template(path = "index/index.html")]
struct IndexTemplate {
    name: String,
    year: String,
}

#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate {
        name: "SphericalKat".to_owned(),
        year: Local::now().date().year().to_string(),
    }
}

#[get("/static/<file..>")]
fn public<'r>(file: PathBuf) -> response::Result<'r> {
    let filename = file.display().to_string();
    Static::get(&filename).map_or_else(
        || Err(Status::NotFound),
        |d| {
            let ext = file
                .as_path()
                .extension()
                .and_then(OsStr::to_str)
                .ok_or_else(|| Status::new(400, "Could not get file extension"))?;
            let content_type = ContentType::from_extension(ext)
                .ok_or_else(|| Status::new(400, "Could not get file content type"))?;
            response::Response::build()
                .header(content_type)
                .sized_body(Cursor::new(d))
                .ok()
        },
    )
}

fn main() {
    rocket::ignite().mount("/", routes!(index, public)).launch();
}
