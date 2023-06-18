use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    db::{error::DbError, Database},
    models::{Id, User, Widget},
};

#[tracing::instrument(skip(db))]
pub async fn create_widget(
    user: User,
    State(db): State<Database>,
    Json(mut widget): Json<Widget>,
) -> Result<Json<Widget>, StatusCode> {
    widget.id = Id::random();
    db.insert_widget(&user, widget)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[tracing::instrument(skip(db))]
pub async fn get_widget(
    user: User,
    Path(widget_id): Path<Id<Widget>>,
    State(db): State<Database>,
) -> Result<Json<Widget>, StatusCode> {
    db.get_widget(&user, &widget_id)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[tracing::instrument(skip(db))]
pub async fn get_widgets(
    user: User,
    State(db): State<Database>,
) -> Result<Json<Vec<Widget>>, StatusCode> {
    db.get_widgets(&user).map(Json).map_err(|e| match e {
        DbError::NotFound => StatusCode::NOT_FOUND,
        DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
    })
}

pub async fn update_widget(
    user: User,
    State(db): State<Database>,
    Json(widget): Json<Widget>,
) -> Result<Json<Widget>, StatusCode> {
    db.update_widget(&user, widget)
        .map(Json)
        .map_err(|e| match e {
            DbError::NotFound => StatusCode::NOT_FOUND,
            DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
            DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
        })
}

pub async fn delete_widget(
    user: User,
    Path(widget_id): Path<Id<Widget>>,
    State(db): State<Database>,
) -> Result<(), StatusCode> {
    db.delete_widget(&user, &widget_id).map_err(|e| match e {
        DbError::NotFound => StatusCode::NOT_FOUND,
        DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
    })
}
