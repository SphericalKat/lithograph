#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rust_embed;

pub mod utils;

use std::{ffi::OsStr, io::Cursor, path::PathBuf};

use askama::Template;
use chrono::{Datelike, Local, NaiveDate};
use comrak::{
    format_html, nodes::NodeValue, parse_document, Arena, ComrakExtensionOptions, ComrakOptions,
};
use lazy_static::lazy_static;
use rocket::{
    http::{ContentType, Status},
    response,
};
use rustc_version_runtime::version;

use crate::utils::{highlight_text, iter_nodes};

lazy_static! {
    static ref EXE: String = std::env::current_exe()
        .unwrap()
        .as_path()
        .to_string_lossy()
        .to_string();
    static ref VERSION: String = version().to_string();
}

#[derive(RustEmbed)]
#[folder = "public/"]
struct Static;

#[derive(RustEmbed)]
#[folder = "posts/"]
struct Posts;

#[derive(Template)]
#[template(path = "index/index.html")]
struct IndexTemplate {
    title: String,
    year: String,
    path: String,
    version: String,
}

struct Post {
    date: String,
    title: String,
    slug: String,
}

#[derive(Template)]
#[template(path = "blog/index.html")]
struct BlogTemplate {
    title: String,
    year: String,
    posts: Vec<Post>,
    path: String,
    version: String,
}

#[derive(Template)]
#[template(path = "blog/post.html")]
struct PostTemplate {
    title: String,
    year: String,
    post: String,
    path: String,
    version: String,
}

#[get("/")]
fn index() -> IndexTemplate {
    IndexTemplate {
        year: Local::now().date().year().to_string(),
        path: EXE.to_string(),
        version: VERSION.to_string(),
        title: "SphericalKat".to_owned(),
    }
}

#[get("/blog")]
fn blog() -> BlogTemplate {
    let mut post_list: Vec<_> = Posts::iter()
        .map(|f| {
            let slug = f.as_ref();
            let split: Vec<_> = slug.splitn(2, '_').collect();
            Post {
                date: split[0].to_owned(),
                title: split[1].replace("-", " ").replace(".md", ""),
                slug: slug.to_owned().replace(".md", ""),
            }
        })
        .collect();
    
    post_list.sort_by(|a, b| {
        let date_a = NaiveDate::parse_from_str(&a.date, "%d-%m-%y").unwrap();
        let date_b = NaiveDate::parse_from_str(&b.date, "%d-%m-%y").unwrap();
        date_b.cmp(&date_a)
    });

    BlogTemplate {
        year: Local::now().date().year().to_string(),
        posts: post_list,
        path: EXE.to_string(),
        version: VERSION.to_string(),
        title: "Blog - SphericalKat".to_owned(),
    }
}

#[get("/blog/<file>")]
fn get_blog<'r>(file: String) -> response::Result<'r> {
    let filename = format!("{}.md", file);
    Posts::get(&filename).map_or_else(
        || Err(Status::NotFound),
        |d| {
            let post_text = String::from_utf8(d.data.as_ref().to_vec()).unwrap();
            let mut opts = &mut ComrakOptions::default();
            opts.extension = ComrakExtensionOptions {
                strikethrough: true,
                tagfilter: false,
                table: true,
                autolink: true,
                tasklist: true,
                superscript: false,
                header_ids: Some("#".to_string()),
                footnotes: false,
                description_lists: false,
                front_matter_delimiter: None,
            };
            opts.render.unsafe_ = true; // needed to embed gists

            let arena = Arena::new();
            let root = parse_document(&arena, &post_text, opts);
            // iter_nodes(root, &|node| match &mut node.data.borrow_mut().value {
            //     &mut NodeValue::CodeBlock(ref mut block) => {
            //         let lang = String::from_utf8(block.info.clone()).unwrap();
            //         let code = String::from_utf8(block.literal.clone()).unwrap();
            //         block.literal = highlight_text(code, lang).as_bytes().to_vec();
            //     }
            //     _ => (),
            // });

            let mut html = vec![];
            format_html(root, opts, &mut html).unwrap();
            response::Response::build()
                .header(ContentType::HTML)
                .sized_body(Cursor::new(
                    PostTemplate {
                        year: Local::now().date().year().to_string(),
                        post: String::from_utf8(html).unwrap(),
                        path: EXE.to_string(),
                        version: VERSION.to_string(),
                        title: file.splitn(2, '_').collect::<Vec<_>>()[1]
                            .to_owned()
                            .replace('-', " "),
                    }
                    .render()
                    .unwrap(),
                ))
                .ok()
        },
    )
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
                .sized_body(Cursor::new(d.data))
                .ok()
        },
    )
}

#[get("/favicon.ico")]
fn favicon<'r>() -> response::Result<'r> {
    let icon = Static::get("favicon.ico").unwrap();
    let content_type = ContentType::Icon;
    response::Response::build()
        .header(content_type)
        .sized_body(Cursor::new(icon.data))
        .ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes!(index, public, blog, get_blog, favicon))
        .launch();
}
