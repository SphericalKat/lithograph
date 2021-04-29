#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    name: String
}

#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate { name: "Amogh".to_owned() }
}

fn main() {
    rocket::ignite().mount("/", routes!(index)).launch();
}
