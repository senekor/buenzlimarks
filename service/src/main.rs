use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Json, Router,
};
use serde_json::json;
use std::{io, net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;

async fn internal_err(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

fn frontend_router() -> Router {
    let app_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("app/dist");

    Router::new().fallback(get_service(ServeDir::new(app_dir)).handle_error(internal_err))
}

#[tokio::main]
async fn main() {
    let http_service = Router::new()
        .route("/api/:name", get(personal_greeting))
        .merge(frontend_router());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(http_service.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Bookmark {
    name: String,
    url: String,
}

async fn personal_greeting(Path(name): Path<String>) -> impl IntoResponse {
    match name.to_lowercase().as_str() {
        "remo" => Json(json!([
            { "name": "Tasks", "url": "https://github.com/users/remlse/projects/1/views/2" },
        ])),
        "silvia" => Json(json!([
            { "name": "Tasks", "url": "https://github.com/users/remlse/projects/1/views/4" },
        ])),
        "harald" => Json(json!([
            { "name": "Requirements", "url": "https://github.com/users/remlse/projects/1/views/6" },
            { "name": "Prioritization", "url": "https://github.com/users/remlse/projects/1/views/7" },
        ])),
        _ => Json(json!([])),
    }
}
