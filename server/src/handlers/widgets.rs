use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    db::{error::DbError, DB},
    models::{id::Id, user::User, widget::Widget},
};

pub async fn create_widget(
    user: User,
    State(db): State<DB>,
    Json(mut widget): Json<Widget>,
) -> Result<Json<Widget>, StatusCode> {
    widget.id = Id::random();
    db.insert_widget(&user.id, widget)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn get_widget(
    user: User,
    Path(widget_id): Path<Id<Widget>>,
    State(db): State<DB>,
) -> Result<Json<Widget>, StatusCode> {
    db.get_widget(&user.id, &widget_id)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        })
}
