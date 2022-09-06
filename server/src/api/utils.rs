use axum::{response::{self, ErrorResponse}, http::StatusCode, Json};
use sea_orm::DbErr;

pub type Payload<T> = response::Result<(StatusCode, Json<T>)>;
pub type NoPayload = response::Result<StatusCode>;

pub fn handle_err(e: DbErr) -> ErrorResponse {
    ErrorResponse::from(match e {
        DbErr::Conn(e) => (StatusCode::INTERNAL_SERVER_ERROR, e + " - Conn"),
        DbErr::Exec(e) => (StatusCode::INTERNAL_SERVER_ERROR, e + " - Exec"),
        DbErr::Query(e) => (StatusCode::BAD_REQUEST, e + " - Query"),
        DbErr::RecordNotFound(e) => (StatusCode::NOT_FOUND, e + " - RecordNotFound"),
        DbErr::Custom(e) => (StatusCode::BAD_REQUEST, e + " - Custom"),
        DbErr::Type(e) => (StatusCode::BAD_REQUEST, e + " - Type"),
        DbErr::Json(e) => (StatusCode::BAD_REQUEST, e + " - Json"),
        DbErr::Migration(e) => (StatusCode::INTERNAL_SERVER_ERROR, e + " - Migration"),
    })
}