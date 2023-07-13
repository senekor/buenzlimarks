use axum::Router;
use buenzlimarks_server::{
    config::Config, docs::docs_handler, frontend::frontend_handler, router::api_router,
};
use clap::Parser;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let config = Config::parse();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from(&config.log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new()
        .nest(
            "/api",
            api_router(&config).layer(TraceLayer::new_for_http()),
        )
        .nest("/docs", Router::new().fallback(docs_handler))
        .fallback(frontend_handler);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    tracing::info!("listening on http://{addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
