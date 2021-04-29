#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

fn main() {
    rocket::ignite().mount("/", routes!());
}
