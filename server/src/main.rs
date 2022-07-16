use axum::{response::Html, routing::get, Router, extract::Path};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/:name", get(personal_greeting));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, Bünzli!</h1>")
}

async fn personal_greeting(Path(name): Path<String>) -> Html<String> {
    Html(format!("<h1>Hello, {name}!</h1>"))
}
