#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
// try to import App from cdylib failed, should use client to wrap it
// use crate::App;
use rocket::http::ContentType;
use rocket::response::Response;
use rocket::{Rocket, Config};

use std::io::Cursor;
use rocket::config::Environment;
use rocket_contrib::serve::StaticFiles;

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

static INDEX_HTML: &str = include_str!("../public/index.html");

fn main() {
  let config = Config::build(Environment::Development)
      .address("0.0.0.0")
      .port(7878)
      .unwrap();

  println!("Rocket server listening on port 7878");

  rocket::custom(config)
      .mount("/", routes![index, favicon, catch_all])
      .mount("/static", StaticFiles::from(format!("{}/public", env!("CARGO_MANIFEST_DIR")).as_str()));
}

/// # Example
///
/// localhost:7878/?init=50
#[get("/?<init>")]
fn index(init: Option<u32>) -> Result<Response<'static>, ()> {
    respond("/".to_string(), init)
}

/// # Example
///
/// localhost:7878/contributors?init=1200
#[get("/<path>?<init>")]
fn catch_all(path: String, init: Option<u32>) -> Result<Response<'static>, ()> {
    respond(path, init)
}

#[get("/favicon.ico")]
fn favicon() -> &'static str {
    ""
}

fn respond(path: String, init: Option<u32>) -> Result<Response<'static>, ()> {
    let app = App::from_json(
        init.unwrap_or(1001),
        path,
    );
    let state = app.store.borrow();

    let html = format!("{}", include_str!("../public/index.html"));
    let html = html.replacen(HTML_PLACEHOLDER, &app.render().to_string(), 1);
    let html = html.replacen(STATE_PLACEHOLDER, &state.to_json(), 1);

    let mut response = Response::new();
    response.set_header(ContentType::HTML);
    response.set_sized_body(Cursor::new(html));

    Ok(response)
}
