use rocket::{Route, fs::FileServer, launch, routes};
use rocket_include_dir::{Dir, StaticFiles, include_dir};

mod auth;
mod backend;
mod db;
mod frontend;
mod modules;

use std::env;

#[launch]
fn rocket() -> _ {
    static PROJECT_DIR: Dir = include_dir!("static");

    let static_routes: Vec<Route> = match env::var("STATIC_DIR").ok() {
        Some(p) => FileServer::from(p).into(),
        None => StaticFiles::from(&PROJECT_DIR).into(),
    };

    rocket::build()
        .mount("/cuteweb", routes![frontend::get_project])
        .mount("/cuteweb/static", static_routes)
        .mount("/cuteweb/api", routes![backend::login_post])
}
