use axum::{routing::get, Router};
use jsonwebtoken::{DecodingKey, Validation};
use models::User;

use crate::state::AppState;

mod config;
mod handlers;
mod state;

pub use config::AuthConfig;
pub use state::AuthState;

use self::handlers::{github_callback, github_login};

pub fn user_from(token: &str, state: &AppState) -> Option<User> {
    jsonwebtoken::decode(
        token,
        &DecodingKey::from_secret(state.auth.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .ok()
    .map(|data| data.claims)
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/github/login", get(github_login))
        .route("/github/callback", get(github_callback))
}
