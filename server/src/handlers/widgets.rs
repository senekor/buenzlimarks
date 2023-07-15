use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use models::{Id, Page, User, Widget};
use serde::Deserialize;

use crate::db::{error::DbError, Database};

#[tracing::instrument(skip(db))]
pub async fn create_widget(
    user: User,
    State(db): State<Database>,
    Json(mut widget): Json<Widget>,
) -> Result<Json<Widget>, StatusCode> {
    widget.id = Id::random();
    db.insert_entity(&user, widget)
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

#[derive(Debug, Deserialize)]
pub struct WidgetFilter {
    page_id: Option<Id<Page>>,
}

#[tracing::instrument(skip(db))]
pub async fn get_widgets(
    user: User,
    State(db): State<Database>,
    query: Query<WidgetFilter>,
) -> Result<Json<Vec<Widget>>, StatusCode> {
    db.get_widgets(&user)
        .map(|mut v| {
            if let Some(page_id) = &query.page_id {
                v.retain(|w| w.page_id == *page_id);
            }
            Json(v)
        })
        .map_err(|e| match e {
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
    db.update_entity(&user, widget)
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
