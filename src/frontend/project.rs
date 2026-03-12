use minijinja::Environment;
use rocket::{State, get, response::content::RawHtml};
use sqlx::SqlitePool;

use crate::auth::Authenticated;

/// GET /cuteweb/project/<project_id>
///
/// Sert la page principale du projet en rendant `layout/main_layout.html.j2`
/// via minijinja. Les sous-templates (main_table, fields, macro…) sont inclus
/// grâce à un loader qui lit dans le répertoire `static/templates`.
#[get("/project/<project_id>")]
pub async fn get_project(
    _auth: Authenticated,
    _pool: &State<SqlitePool>,
    env: &State<Environment<'_>>,
    project_id: i64,
) -> RawHtml<String> {
    let html = render_layout(project_id, env)
        .unwrap_or_else(|e| format!("<pre>Erreur de template : {e}</pre>"));
    RawHtml(html)
}

/// Construit l'environnement minijinja avec tous les templates du répertoire,
/// puis rend `layout/main_layout.html.j2` avec le contexte donné.
fn render_layout(project_id: i64, env: &Environment) -> Result<String, minijinja::Error> {
    // Contexte minimal : project_id
    let ctx = serde_json::json!({ "project_id": project_id });

    env.get_template("layout/main_layout.html.j2")?.render(&ctx)
}
