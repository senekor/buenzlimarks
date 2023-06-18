use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};

use crate::{
    db::Database,
    models::{AuthProvider, Id, Settings, User},
    state::AppState,
};

mod config;
mod state;

pub use config::AuthConfig;
pub use state::AuthState;

pub fn user_from(token: &str, _state: &AppState) -> Option<User> {
    Some(User {
        id: token.into(),
        provider: AuthProvider::Dev,
    })
}

#[tracing::instrument(skip(db))]
pub async fn login(
    Path(user_id): Path<Id<User>>,
    State(db): State<Database>,
    State(auth): State<AuthState>,
) -> Result<String, StatusCode> {
    let user = User {
        id: user_id,
        provider: AuthProvider::Dev,
    };
    if !db.contains_user(&user) {
        tracing::debug!("login of new user {}, inserting...", user.id);
        let settings = Settings {
            name: user.id.to_string(),
        };
        if let Err(e) = db.insert_user(&user, settings) {
            tracing::error!("Database failed to insert user: {user:?} {e:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    tracing::debug!("login succeeded for user {user:?}");

    Ok(user.id.into())
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/login/:user_id", get(login))
}
