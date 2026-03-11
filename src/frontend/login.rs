use rocket::{State, get};
use sqlx::SqlitePool;

use crate::auth::Authenticated;

#[get("/cuteweb/login")]
pub async fn login(auth: Authenticated, pool: &State<SqlitePool>) -> String {
    // Afficher une page de login à partir d'un template
    "".to_string()
}
