use axum::{routing::get, Json, Router};

use crate::{db::DB, models::user::User};

async fn whoami(user: User) -> Json<User> {
    Json(user)
}

pub fn routes() -> Router<DB> {
    Router::<DB>::new().route("/me", get(whoami))
}
