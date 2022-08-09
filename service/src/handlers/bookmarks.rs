use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use sea_orm::*;

use crate::entities::bookmarks;

use super::{handle_err, HandlerResult};

async fn get_bookmarks(Extension(db): Extension<DatabaseConnection>) -> impl IntoResponse {
    bookmarks::Entity::find()
        .into_json()
        .all(&db)
        .await
        .map(|m| (StatusCode::OK, Json(m)))
        .map_err(handle_err)
}

async fn add_bookmark(
    Extension(db): Extension<DatabaseConnection>,
    Json(json): Json<JsonValue>,
) -> HandlerResult<Json<bookmarks::Model>> {
    bookmarks::ActiveModel::new_from_json(json)
        .map_err(handle_err)?
        .insert(&db)
        .await
        .map(|m| (StatusCode::OK, Json(m)))
        .map_err(handle_err)
}

async fn update_bookmark(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<String>,
    Json(json): Json<JsonValue>,
) -> impl IntoResponse {
    bookmarks::ActiveModel::from_id_and_json(id, json)
        .map_err(handle_err)?
        .update(&db)
        .await
        .map(|m| (StatusCode::OK, Json(m)))
        .map_err(handle_err)
}

async fn delete_bookmark(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    bookmarks::Entity::delete_by_id(id)
        .exec(&db)
        .await
        .map(|_| StatusCode::OK)
        .map_err(handle_err)
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_bookmarks).post(add_bookmark))
        .route("/:id", put(update_bookmark).delete(delete_bookmark))
}
