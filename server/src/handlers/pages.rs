use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    db::{error::DbError, Database},
    models::{Id, Page, User},
};

#[tracing::instrument(skip(db))]
pub async fn create_page(
    user: User,
    State(db): State<Database>,
    Json(mut page): Json<Page>,
) -> Result<Json<Page>, StatusCode> {
    page.id = Id::random();
    db.insert_page(&user, page).map(Json).map_err(|e| match e {
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
    db.get_page(&user, &page_id).map(Json).map_err(|e| match e {
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
    db.get_pages(&user).map(Json).map_err(|e| match e {
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
    db.update_page(&user, page).map(Json).map_err(|e| match e {
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
    db.delete_page(&user, &page_id).map_err(|e| match e {
        DbError::NotFound => StatusCode::NOT_FOUND,
        DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
    })
}
