use minijinja::Environment;
use rocket::{Route, fs::FileServer, launch, routes};
use rocket_include_dir::{Dir, StaticFiles, include_dir};

mod auth;
mod backend;
mod db;
mod frontend;
mod modules;

use std::env;

#[launch]
async fn rocket() -> _ {
    static PROJECT_DIR: Dir = include_dir!("static");

    // Permet de servir les routes statiques.
    let static_routes: Vec<Route> = match env::var("STATIC_DIR").ok() {
        Some(p) => FileServer::from(p).into(),
        None => StaticFiles::from(&PROJECT_DIR).into(),
    };
    /// Répertoire statique embarqué dans le binaire.
    static TEMPLATES_DIR: Dir = include_dir!("static/templates");

    let pool = db::init_db()
        .await
        .expect("Impossible d'initialiser la base de données");

    let mut environment: Environment = Environment::new();
    environment.set_loader(|name| {
        if let Some(file) = TEMPLATES_DIR.get_file(name) {
            if let Some(content) = file.contents_utf8() {
                eprintln!("Invoked with {}", name);
                Ok(Some(content.to_string()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    });

    rocket::build()
        // Pages HTML
        .mount(
            "/cuteweb",
            routes![frontend::get_project, frontend::login_get],
        )
        // Fichiers statiques
        .mount("/cuteweb/static", static_routes)
        // API : authentification
        .mount(
            "/cuteweb/api",
            routes![backend::login_post, backend::setvar_post,],
        )
        // API : récupération des données des modules
        .mount(
            "/cuteweb/api/retrieve",
            routes![modules::main_table_get, modules::fields_get,],
        )
        .manage(pool)
        .manage(environment)
}
