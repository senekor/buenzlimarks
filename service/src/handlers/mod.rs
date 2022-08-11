use axum::{Extension, Router};
use sea_orm::*;

use crate::migrations::{Migrator, MigratorTrait};

mod auth;
mod bookmarks;
mod utils;

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
