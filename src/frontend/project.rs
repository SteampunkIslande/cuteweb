use rocket::{State, get};
use sqlx::SqlitePool;

use crate::auth::Authenticated;

#[get("/cuteweb/project/<project_id>")]
pub async fn get_project(auth: Authenticated, pool: &State<SqlitePool>, project_id: i64) -> String {
    match auth.user {
        Some(u) => {
            format!("Hello, {}", u.username)
        }
        None => {
            format!("Hello, anonymous!")
        }
    }
}
