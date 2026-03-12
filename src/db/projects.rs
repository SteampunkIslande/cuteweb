use std::collections::HashMap;

use sqlx::{Row, SqlitePool};

/// Récupère les variables utilisateur d'un projet sous forme de HashMap<String, String>.
/// Le champ `user_vars` est stocké en JSON dans SQLite.
pub async fn get_project_uservars(
    project_id: i64,
    pool: &SqlitePool,
) -> Result<HashMap<String, String>, anyhow::Error> {
    let row = sqlx::query("SELECT user_vars FROM projects WHERE project_id = ?")
        .bind(project_id)
        .fetch_optional(pool)
        .await?;

    match row {
        None => Ok(HashMap::new()),
        Some(r) => {
            let json_str: String = r.try_get("user_vars")?;
            let map: HashMap<String, String> = serde_json::from_str(&json_str).unwrap_or_default();
            Ok(map)
        }
    }
}

/// Met à jour (ou insère) une variable utilisateur pour un projet donné.
/// Fusionne la nouvelle clé/valeur dans le JSON existant.
pub async fn set_project_uservar(
    project_id: i64,
    key: &str,
    value: &str,
    pool: &SqlitePool,
) -> Result<(), anyhow::Error> {
    // Upsert : on crée la ligne si elle n'existe pas encore
    sqlx::query(
        "INSERT INTO projects (project_id, user_vars) VALUES (?, '{}')
         ON CONFLICT(project_id) DO NOTHING",
    )
    .bind(project_id)
    .execute(pool)
    .await?;

    // Lire le JSON courant
    let row = sqlx::query("SELECT user_vars FROM projects WHERE project_id = ?")
        .bind(project_id)
        .fetch_one(pool)
        .await?;

    let json_str: String = row.try_get("user_vars")?;
    let mut map: HashMap<String, String> = serde_json::from_str(&json_str).unwrap_or_default();
    map.insert(key.to_string(), value.to_string());

    let new_json = serde_json::to_string(&map)?;

    sqlx::query("UPDATE projects SET user_vars = ? WHERE project_id = ?")
        .bind(new_json)
        .bind(project_id)
        .execute(pool)
        .await?;

    Ok(())
}
