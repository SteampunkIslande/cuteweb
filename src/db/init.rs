use cuteweb::get_config;
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::time::Duration;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url: String = get_config().db;
    init_db_from_url(&database_url).await
}

pub async fn create_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

pub async fn init_db_from_url(url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = create_pool(url).await?;
    Ok(pool)
}
