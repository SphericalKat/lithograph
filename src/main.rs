#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rust_embed;

pub mod utils;

use std::{ffi::OsStr, io::Cursor, path::PathBuf, collections::HashMap};

use askama::Template;
use chrono::{Datelike, Local, NaiveDate};
use comrak::{
    format_html, parse_document, Arena, ComrakExtensionOptions, ComrakOptions,
};
use lazy_static::lazy_static;
use markdown_meta_parser::MetaData;
use rocket::{
    http::{ContentType, Status},
    response,
};
use rustc_version_runtime::version;


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
    blurb: String,
    tags: Vec<String>,
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

            let filename = f.as_ref();

            let d = Posts::get(&filename).unwrap();
            let content = String::from_utf8(d.data.as_ref().to_vec()).unwrap();

            let mut type_mark = HashMap::new();
            type_mark.insert("tags".into(), "array");

            let meta = MetaData {
                content,
                required: vec!["title".to_owned(), "tags".to_owned(), "date".to_owned(), "blurb".to_owned()],
                type_mark  
            };

            let (parsed_meta, _) = meta.parse().unwrap();

            let title = match parsed_meta["title"].clone() {
                markdown_meta_parser::Value::String(t) => t.replace("'", ""),
                _ => "".to_owned()
            };

            let date = match parsed_meta["date"].clone() {
                markdown_meta_parser::Value::String(d) => d,
                _ => "".to_owned()
            };

            let blurb = match parsed_meta["blurb"].clone() {
                markdown_meta_parser::Value::String(b) => b.replace('"', ""),
                _ => "".to_owned()
            };

            let tags = match parsed_meta["tags"].clone() {
                markdown_meta_parser::Value::Array(t) => t,
                _ => vec![]
            };

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
            let root = parse_document(&arena, &blurb, opts);

            let mut html = vec![];
            format_html(root, opts, &mut html).unwrap();

            Post {
                date,
                title,
                slug: f.as_ref().to_owned().replace(".md", ""),
                blurb: String::from_utf8(html).unwrap(),
                tags,
            }
        })
        .collect();
    
    post_list.sort_by(|a, b| {
        let date_a = NaiveDate::parse_from_str(&a.date, "%Y-%m-%d").unwrap();
        let date_b = NaiveDate::parse_from_str(&b.date, "%Y-%m-%d").unwrap();
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

            let mut type_mark = HashMap::new();
            type_mark.insert("tags".into(), "array");

            let meta = MetaData {
                content: post_text,
                required: vec!["title".to_owned(), "tags".to_owned(), "date".to_owned(), "blurb".to_owned()],
                type_mark  
            };

            let (parsed_meta, body) = meta.parse().unwrap();

            let title = match parsed_meta["title"].clone() {
                markdown_meta_parser::Value::String(t) => t,
                _ => "".to_owned()
            };

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
            let root = parse_document(&arena, &body, opts);

            let mut html = vec![];
            format_html(root, opts, &mut html).unwrap();

            response::Response::build()
                .header(ContentType::HTML)
                .sized_body(Cursor::new(
                    PostTemplate {
                        title,
                        year: Local::now().date().year().to_string(),
                        post: String::from_utf8(html).unwrap(),
                        path: EXE.to_string(),
                        version: VERSION.to_string(),
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
