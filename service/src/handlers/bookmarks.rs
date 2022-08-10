use axum::{
    extract::Path,
    http::StatusCode,
    response::ErrorResponse,
    routing::{get, put},
    Extension, Json, Router,
};
use sea_orm::*;

use crate::entities::{bookmarks, Bookmark, User};

use super::{handle_err, NoPayload, Payload};

async fn get_bookmarks(
    user: User,
    Extension(db): Extension<DatabaseConnection>,
) -> Payload<Vec<Bookmark>> {
    bookmarks::Entity::find()
        .filter(bookmarks::Column::UserId.eq(user.id))
        .all(&db)
        .await
        .map(|m| (StatusCode::OK, Json(m)))
        .map_err(handle_err)
}

async fn add_bookmark(
    user: User,
    Json(json): Json<JsonValue>,
    Extension(db): Extension<DatabaseConnection>,
) -> Payload<Bookmark> {
    bookmarks::ActiveModel::new_from_json(user.id, json)
        .map_err(handle_err)?
        .insert(&db)
        .await
        .map(|m| (StatusCode::OK, Json(m)))
        .map_err(handle_err)
}

async fn update_bookmark(
    user: User,
    Path(id): Path<String>,
    Json(json): Json<JsonValue>,
    Extension(db): Extension<DatabaseConnection>,
) -> Payload<Bookmark> {
    bookmarks::Entity::find_by_id(id.clone())
        .filter(bookmarks::Column::UserId.eq(user.id.clone()))
        .one(&db)
        .await
        .map_err(handle_err)?
        .ok_or_else(|| ErrorResponse::from(StatusCode::NOT_FOUND))?;

    bookmarks::ActiveModel::parse(id, user.id, json)
        .map_err(handle_err)?
        .update(&db)
        .await
        .map(|m| (StatusCode::OK, Json(m)))
        .map_err(handle_err)
}

async fn delete_bookmark(
    user: User,
    Path(id): Path<String>,
    Extension(db): Extension<DatabaseConnection>,
) -> NoPayload {
    bookmarks::Entity::find_by_id(id.clone())
        .filter(bookmarks::Column::UserId.eq(user.id))
        .one(&db)
        .await
        .map_err(handle_err)?
        .ok_or_else(|| ErrorResponse::from(StatusCode::NOT_FOUND))?
        .delete(&db)
        .await
        .map(|_| StatusCode::OK)
        .map_err(handle_err)
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(get_bookmarks).post(add_bookmark))
        .route("/:id", put(update_bookmark).delete(delete_bookmark))
}
