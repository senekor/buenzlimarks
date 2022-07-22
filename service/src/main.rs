use axum::{
    extract::Path,
    http::StatusCode,
    response::Html,
    response::IntoResponse,
    routing::{get, get_service},
    Json, Router,
};
use serde_json::json;
use std::{io, net::SocketAddr, path::PathBuf};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    // access for files
    let frontend_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("app/dist");

    // build our application with a route
    let app = Router::new()
        .route("/api", get(general_greeting))
        .route("/api/:name", get(personal_greeting))
        .fallback(get_service(ServeDir::new(frontend_dir)).handle_error(handle_error));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn general_greeting() -> Html<&'static str> {
    Html("<h1>Hello, Bünzli!</h1>")
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
        _ => Json(json!({})),
    }
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
