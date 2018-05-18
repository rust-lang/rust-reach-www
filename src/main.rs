#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::response::NamedFile;

#[derive(Serialize)]
struct Context {
  parent: String,
}

#[get("/")]
fn index() -> Template {
    let page = "index".to_string();
    let context = Context {
      parent: "layout".to_string(),
    };
    Template::render(page, &context)
}

#[get("/2018/participants")]
fn participants() -> Template {
    let page = "participants-index".to_string();
    let context = Context {
      parent: "layout".to_string(),
    };
    Template::render(page, &context)
}

#[get("/static/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite()
      .attach(Template::fairing())
      .mount("/", routes![index, participants, files]).launch();
}
