use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    db::{error::DbError, DB},
    models::{id::Id, page::Page, user::User},
};

pub async fn create_page(
    user: User,
    State(db): State<DB>,
    Json(mut page): Json<Page>,
) -> Result<Json<Page>, StatusCode> {
    page.id = Id::random();
    db.insert_page(&user.id, page)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn get_page(
    user: User,
    Path(page_id): Path<Id<Page>>,
    State(db): State<DB>,
) -> Result<Json<Page>, StatusCode> {
    db.get_page(&user.id, &page_id)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn get_pages(user: User, State(db): State<DB>) -> (StatusCode, Json<Vec<Page>>) {
    match db.get_pages(&user.id) {
        Ok(pages) => (StatusCode::OK, Json(pages)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}
