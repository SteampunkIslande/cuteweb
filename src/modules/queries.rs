use std::collections::HashMap;

use minijinja::Environment;

/// Erreur de rendu de requête
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Erreur de rendu de template: {0}")]
    Template(#[from] minijinja::Error),
    #[error("Variable utilisateur manquante: {var}. {hint}")]
    MissingVar { var: String, hint: String },
}

/// Ensemble des variantes de requêtes DuckDB pour le module main_table.
/// Chaque variante embarque son template SQL Jinja2.
pub enum Queries {
    /// Requête par défaut : SELECT des champs demandés sur un parquet
    Default,
    /// Variante avec LEFT JOINs entre plusieurs runs
    WithLeftJoins,
}

impl Queries {
    /// Retourne le template SQL Jinja2 associé à cette variante.
    fn template(&self) -> &'static str {
        match self {
            Queries::Default => concat!(
                "SELECT {{ fields | join(', ') }} ",
                "FROM read_parquet('{{ datalake_root }}/runs/{{ run }}.parquet') ",
                "WHERE sample_name IN ({{ samples | join(', ') }}) ",
                "LIMIT {{ limit | default(100) }} ",
                "OFFSET {{ offset | default(0) }}"
            ),
            Queries::WithLeftJoins => concat!(
                "SELECT base.variant_hash, {{ fields | join(', ') }} ",
                "FROM read_parquet('{{ datalake_root }}/runs/{{ runs[0] }}.parquet') AS base ",
                "{% for run in runs[1:] %}",
                "LEFT JOIN read_parquet('{{ datalake_root }}/runs/{{ run }}.parquet') ",
                "    AS r{{ loop.index }} ON base.variant_hash = r{{ loop.index }}.variant_hash ",
                "{% endfor %}",
                "LIMIT {{ limit | default(100) }} ",
                "OFFSET {{ offset | default(0) }}"
            ),
        }
    }

    /// Rend le template SQL avec les variables utilisateur fournies.
    ///
    /// Les valeurs stockées dans `vars` peuvent être :
    /// - des chaînes simples  (ex: `"datalake_root"` → `"/data"`)
    /// - des tableaux JSON    (ex: `"fields"` → `'["chrom","pos","ref"]'`)
    /// - des entiers JSON     (ex: `"limit"` → `"500"`)
    ///
    /// Retourne une erreur si une variable requise est absente.
    pub fn render(&self, vars: &HashMap<String, String>) -> Result<String, RenderError> {
        let mut env = Environment::new();
        env.add_template("query", self.template())
            .map_err(RenderError::Template)?;

        // Construire le contexte : chaque valeur est parsée en serde_json::Value
        // afin de conserver les listes et les nombres natifs pour minijinja.
        let ctx: serde_json::Map<String, serde_json::Value> = vars
            .iter()
            .map(|(k, v)| {
                let jv: serde_json::Value =
                    serde_json::from_str(v).unwrap_or(serde_json::Value::String(v.clone()));
                (k.clone(), jv)
            })
            .collect();

        env.get_template("query")
            .map_err(RenderError::Template)?
            .render(&ctx)
            .map_err(|e| {
                let msg = e.to_string();
                // Détecter une variable manquante pour afficher un message utile au frontend
                if msg.contains("undefined variable") {
                    let var = msg.split('`').nth(1).unwrap_or("inconnue").to_string();
                    RenderError::MissingVar {
                        hint: format!(
                            "Veuillez définir « {} » dans les paramètres du projet.",
                            var
                        ),
                        var,
                    }
                } else {
                    RenderError::Template(e)
                }
            })
    }
}
