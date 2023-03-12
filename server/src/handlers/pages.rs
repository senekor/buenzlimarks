use axum::{extract::{State, Path}, http::StatusCode, Json};

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
    db.read_page(&user.id, &page_id).map(Json).map_err(|e| match e {
        DbError::NotFound => StatusCode::NOT_FOUND,
        DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
    })
}
