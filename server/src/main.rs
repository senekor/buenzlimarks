use axum::Router;
use std::net::SocketAddr;

use lib::{db, handlers};

mod frontend;

#[tokio::main]
async fn main() {
    let db = db::get();

    let http_service = Router::new()
        .nest("/api", handlers::routes(db))
        .merge(frontend::frontend_routes());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(http_service.into_make_service())
        .await
        .unwrap();
}
