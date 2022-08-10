use axum::{
    http::StatusCode,
    response::{self, ErrorResponse},
    Extension, Json, Router,
};
use sea_orm::*;

use crate::migrations::{Migrator, MigratorTrait};

mod auth;
mod bookmarks;

type Payload<T> = response::Result<(StatusCode, Json<T>)>;
type NoPayload = response::Result<StatusCode>;

fn handle_err(e: DbErr) -> ErrorResponse {
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

pub async fn api_routes() -> Router {
    dotenv::dotenv().ok();
    let db_ulr = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let conn = Database::connect(db_ulr)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    Router::new()
        .nest("/auth", auth::routes())
        .nest("/bookmarks", bookmarks::routes())
        .layer(Extension(conn))
        .layer(Extension(auth::jwt_key()))
}
