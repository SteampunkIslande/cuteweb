use std::collections::HashMap;

use duckdb::Connection;
use rocket::{State, get, serde::json::Json};
use sqlx::SqlitePool;

use cuteweb::ApiResponse;

use crate::db::get_project_uservars;
use crate::modules::queries::{Queries, RenderError};

/// Représente une ligne renvoyée par DuckDB (colonnes → valeurs en String).
type Row = HashMap<String, String>;

/// GET /cuteweb/api/retrieve/main_table/<project_id>
///
/// Récupère les user_vars du projet, choisit la variante de requête,
/// rend le SQL via le template Jinja, l'exécute sur DuckDB et retourne
/// les résultats sous forme d'ApiResponse JSON.
#[get("/main_table/<project_id>")]
pub async fn main_table_get(
    project_id: i64,
    pool: &State<SqlitePool>,
) -> Json<ApiResponse<Vec<Row>>> {
    let vars = match get_project_uservars(project_id, pool).await {
        Ok(v) => v,
        Err(e) => return Json(ApiResponse::error(e.to_string())),
    };

    // Choisir la variante de requête selon la présence de `uses_left_joins`
    let query_variant = if vars.contains_key("uses_left_joins") {
        Queries::WithLeftJoins
    } else {
        Queries::Default
    };

    let sql = match query_variant.render(&vars) {
        Ok(q) => q,
        Err(RenderError::MissingVar { var, hint }) => {
            return Json(ApiResponse::error(format!(
                "Variable manquante : {var}. {hint}"
            )));
        }
        Err(e) => return Json(ApiResponse::error(e.to_string())),
    };

    // Exécuter la requête DuckDB (connexion in-memory pour les tests bateau)
    match run_duckdb_query(&sql) {
        Ok(rows) => Json(ApiResponse::success(rows)),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}

/// Exécute un SQL DuckDB et retourne les lignes sous forme de Vec<HashMap<String,String>>.
/// Chaque valeur est convertie en String via `duckdb::types::ValueRef::to_string()`.
fn run_duckdb_query(sql: &str) -> Result<Vec<Row>, duckdb::Error> {
    let conn = Connection::open_in_memory()?;
    let mut stmt = conn.prepare(sql)?;

    let col_names: Vec<String> = stmt.column_names();
    let col_count = col_names.len();

    let rows: Vec<Row> = stmt
        .query_map([], |row| {
            let mut map = HashMap::<String, String>::new();
            for i in 0..col_count {
                // Récupérer la valeur DuckDB comme String
                let val: String = row
                    .get::<_, duckdb::types::Value>(i)
                    .map(|v| format!("{v:?}"))
                    .unwrap_or_else(|_| "NULL".to_string());
                map.insert(col_names[i].clone(), val);
            }
            Ok(map)
        })?
        .filter_map(Result::ok)
        .collect();

    Ok(rows)
}
