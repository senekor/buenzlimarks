use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};

use crate::db::{BuenzlimarksDatabase, DB};

async fn get_bookmarks(
    // user: User,
    State(db): State<DB>,
) -> impl IntoResponse {
    Json(db.get_bookmarks("dev"))
}

pub fn routes() -> Router<DB> {
    Router::<DB>::new().route("/", get(get_bookmarks))
}
