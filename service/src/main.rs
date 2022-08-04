use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Extension, Router};
use sea_orm::*;
use std::{io, net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;

use lib::{
    handlers::api_routes,
    migrations::{Migrator, MigratorTrait},
};

async fn internal_err(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

fn frontend_routes() -> Router {
    let dist = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../app/dist");
    Router::new().fallback(get_service(ServeDir::new(dist)).handle_error(internal_err))
}

#[tokio::main]
async fn main() {
    let env_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../dev/env.sh");
    dotenv::from_path(env_path).ok();
    let db_ulr = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let conn = Database::connect(db_ulr)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let http_service = Router::new()
        .nest("/api", api_routes())
        .layer(Extension(conn))
        .merge(frontend_routes());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(http_service.into_make_service())
        .await
        .unwrap();
}
