use axum::Router;
use clap::Parser;
use lib::{config::Config, frontend::frontend_router, router::api_router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let router = Router::new()
        .nest("/api", api_router(&config))
        .merge(frontend_router());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
