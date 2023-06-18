use axum::extract::FromRef;

use crate::{db::Database, handlers::auth::AuthState};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
    pub auth: AuthState,
}

impl AppState {
    pub fn new(config: &crate::config::Config) -> Self {
        Self {
            db: crate::db::get(&config.db),
            auth: AuthState::new(&config.auth),
        }
    }
}

impl FromRef<AppState> for Database {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for AuthState {
    fn from_ref(state: &AppState) -> Self {
        state.auth.clone()
    }
}
