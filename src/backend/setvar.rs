use rocket::{State, post, serde::json::Json};
use sqlx::SqlitePool;

use cuteweb::ApiResponse;

use crate::db::set_project_uservar;

/// Corps de la requête POST /cuteweb/api/setvar
#[derive(Debug, serde::Deserialize)]
pub struct UserVarSetter {
    /// Identifiant du projet
    pub project_id: i64,
    /// Nom de la variable à mettre à jour (ex: "fields", "samples", "limit")
    pub key: String,
    /// Valeur sérialisée en JSON string (ex: `'["chrom","pos"]'` ou `"500"`)
    pub value: String,
}

/// POST /cuteweb/api/setvar
///
/// Met à jour une variable utilisateur dans la table `projects`.
/// Le frontend appelle cet endpoint après chaque interaction utilisateur
/// (ex: cocher/décocher un champ dans le widget fields).
#[post("/setvar", data = "<body>")]
pub async fn setvar_post(
    body: Json<UserVarSetter>,
    pool: &State<SqlitePool>,
) -> Json<ApiResponse<()>> {
    match set_project_uservar(body.project_id, &body.key, &body.value, pool).await {
        Ok(()) => Json(ApiResponse::success(())),
        Err(e) => Json(ApiResponse::error(e.to_string())),
    }
}
