use rocket::{State, get, serde::json::Json};
use sqlx::SqlitePool;

use cuteweb::ApiResponse;

use crate::db::get_project_uservars;

/// GET /cuteweb/api/retrieve/fields/<project_id>
///
/// Retourne la liste des champs (colonnes) disponibles pour ce projet.
/// La liste est stockée dans `user_vars["fields"]` sous forme de tableau JSON.
/// Si `fields` n'est pas encore défini, retourne une liste par défaut.
#[get("/fields/<project_id>")]
pub async fn fields_get(
    project_id: i64,
    pool: &State<SqlitePool>,
) -> Json<ApiResponse<Vec<String>>> {
    let vars = match get_project_uservars(project_id, pool).await {
        Ok(v) => v,
        Err(e) => return Json(ApiResponse::error(e.to_string())),
    };

    let fields: Vec<String> = match vars.get("fields") {
        Some(json_str) => serde_json::from_str(json_str).unwrap_or_else(|_| default_fields()),
        None => default_fields(),
    };

    Json(ApiResponse::success(fields))
}

/// Champs disponibles par défaut (exemple bateau pour le module génétique).
fn default_fields() -> Vec<String> {
    vec![
        "chrom".to_string(),
        "pos".to_string(),
        "ref".to_string(),
        "alt".to_string(),
        "sample_name".to_string(),
        "genotype".to_string(),
        "variant_hash".to_string(),
    ]
}
