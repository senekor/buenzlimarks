use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use std::{env, io, net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;

use lib::{db, handlers};

async fn internal_err(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

fn frontend_routes() -> Router {
    let dist = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../app/dist");
    Router::new().fallback_service(get_service(ServeDir::new(dist)).handle_error(internal_err))
}

#[tokio::main]
async fn main() {
    let db = db::get();

    let http_service = Router::new()
        .nest("/api", handlers::routes(db))
        .merge(frontend_routes());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(http_service.into_make_service())
        .await
        .unwrap();
}
