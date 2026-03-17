use cuteweb::config;
use rocket::request::{FromRequest, Outcome, Request};
use sqlx::SqlitePool;

use super::AuthError;
use super::User;

pub struct Authenticated {
    pub user: Option<User>,
}

async fn user_from_cookie(request: &Request<'_>) -> Option<User> {
    // If we are running as the local user, there is no need to authenticate
    if config::get_config().local {
        Some(User::default())
    } else {
        let cookies = request.cookies();

        let pool = match request.rocket().state::<SqlitePool>() {
            None => {
                eprintln!("Cannot connect to database...");
                return None;
            }
            Some(pool) => pool,
        };

        match cookies.get_private("user_id") {
            Some(cookie) => {
                let user_id: i64 = cookie.value().parse().ok()?;
                User::find_by_id(user_id, pool).await.ok()?
            }
            None => None,
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authenticated {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Vérifie si l'utilisateur est authentifié via un cookie de session
        Outcome::Success(Authenticated {
            user: user_from_cookie(request).await,
        })
    }
}
