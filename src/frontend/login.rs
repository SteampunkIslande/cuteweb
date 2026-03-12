use minijinja::{Environment, context};
use rocket::response::content::RawHtml;
use rocket::{State, get};

#[get("/login")]
pub async fn login_get(env: &State<Environment<'static>>) -> RawHtml<String> {
    let ctx = context! { error => None::<String> };

    let template = if let Ok(template) = env.get_template("login.html.j2") {
        template
    } else {
        return RawHtml("Impossible de charger le template login.html.j2".to_string());
    };

    RawHtml(
        template
            .render(ctx)
            .unwrap_or_else(|_| "Template error".to_string()),
    )
}
