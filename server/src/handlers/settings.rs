use axum::{extract::State, http::StatusCode, Json};

use crate::{
    db::{error::DbError, Database},
    models::{Settings, User},
};

#[tracing::instrument(skip(db))]
pub async fn settings(
    user: User,
    State(db): State<Database>,
) -> Result<Json<Settings>, StatusCode> {
    db.get_settings(&user).map(Json).map_err(|e| match e {
        DbError::NotFound => StatusCode::NOT_FOUND,
        DbError::WhoopsieDoopsie => StatusCode::INTERNAL_SERVER_ERROR,
        DbError::AlreadyExists => StatusCode::INTERNAL_SERVER_ERROR,
    })
}
