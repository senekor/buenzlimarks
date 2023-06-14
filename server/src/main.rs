use axum::Router;
use lib::{frontend::frontend_router, router::api_router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .nest("/api", api_router())
        .merge(frontend_router());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
