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
    run_migrations(&pool).await?;
    Ok(pool)
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS Projects (
            project_id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_vars  TEXT NOT NULL DEFAULT '{}'
        );
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS Users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            usermail TEXT NOT NULL UNIQUE,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL,
            last_login TEXT,
            is_admin INTEGER NOT NULL DEFAULT 0
        );
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
