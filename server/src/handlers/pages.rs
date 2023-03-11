use axum::{extract::State, http::StatusCode, Json};

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
