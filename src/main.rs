#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use askama::Template;
use rocket_contrib::serve::StaticFiles;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String,
}

#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate {
        name: "Amogh".to_owned(),
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes!(index))
        .mount("/static", StaticFiles::from("/Users/sphericalkat/CLionProjects/lithograph/public"))
        .launch();
}
