use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use models::{Id, Page, User};

use crate::db::{error::DbError, Database};

#[tracing::instrument(skip(db))]
pub async fn create_page(
    user: User,
    State(db): State<Database>,
    Json(mut page): Json<Page>,
) -> Result<Json<Page>, StatusCode> {
    page.id = Id::random();
    db.insert_entity(&user, page)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[tracing::instrument(skip(db))]
pub async fn get_page(
    user: User,
    Path(page_id): Path<Id<Page>>,
    State(db): State<Database>,
) -> Result<Json<Page>, StatusCode> {
    db.get_entity(&user, &page_id)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[tracing::instrument(skip(db))]
pub async fn get_pages(
    user: User,
    State(db): State<Database>,
) -> Result<Json<Vec<Page>>, StatusCode> {
    db.get_entities(&user).map(Json).map_err(|e| match e {
        DbError::NotFound => StatusCode::NOT_FOUND,
        DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
    })
}

pub async fn update_page(
    user: User,
    State(db): State<Database>,
    Json(page): Json<Page>,
) -> Result<Json<Page>, StatusCode> {
    db.update_entity(&user, page)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn delete_page(
    user: User,
    Path(page_id): Path<Id<Page>>,
    State(db): State<Database>,
) -> Result<(), StatusCode> {
    db.delete_entity(&user, &page_id).map_err(|e| match e {
        DbError::NotFound => StatusCode::NOT_FOUND,
        DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
    })
}
