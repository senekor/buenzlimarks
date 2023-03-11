use axum::{extract::State, http::StatusCode, Json};

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
