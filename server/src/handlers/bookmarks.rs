use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use models::{sanitize_bookmark, Bookmark, Id, User, Widget};
use serde::Deserialize;

use crate::db::{error::DbError, Database};

#[tracing::instrument(skip(db))]
pub async fn create_bookmark(
    user: User,
    State(db): State<Database>,
    Json(mut bookmark): Json<Bookmark>,
) -> Result<Json<Bookmark>, StatusCode> {
    tracing::debug!("create bookmark");
    bookmark.id = Id::random();
    sanitize_bookmark(&mut bookmark);
    db.insert_entity(&user, bookmark)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[tracing::instrument(skip(db))]
pub async fn get_bookmark(
    user: User,
    Path(bookmark_id): Path<Id<Bookmark>>,
    State(db): State<Database>,
) -> Result<Json<Bookmark>, StatusCode> {
    db.get_entity(&user, &bookmark_id)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[derive(Debug, Deserialize)]
pub struct BookmarkFilter {
    widget_id: Option<Id<Widget>>,
}

#[tracing::instrument(skip(db))]
pub async fn get_bookmarks(
    user: User,
    State(db): State<Database>,
    query: Query<BookmarkFilter>,
) -> Result<Json<Vec<Bookmark>>, StatusCode> {
    db.get_bookmarks(&user)
        .map(|mut v| {
            if let Some(widget_id) = &query.widget_id {
                v.retain(|b| b.widget_id == *widget_id);
            }
            Json(v)
        })
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[tracing::instrument(skip(db))]
pub async fn update_bookmark(
    user: User,
    State(db): State<Database>,
    Json(mut bookmark): Json<Bookmark>,
) -> Result<Json<Bookmark>, StatusCode> {
    sanitize_bookmark(&mut bookmark);
    db.update_entity(&user, bookmark)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[tracing::instrument(skip(db))]
pub async fn delete_bookmark(
    user: User,
    Path(bookmark_id): Path<Id<Bookmark>>,
    State(db): State<Database>,
) -> Result<(), StatusCode> {
    match db.delete_bookmark(&user, &bookmark_id) {
        Ok(_) => Ok(()),
        Err(DbError::NotFound) => Err(StatusCode::NOT_FOUND),
        Err(DbError::WhoopsieDoopsie) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        Err(DbError::AlreadyExists) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
