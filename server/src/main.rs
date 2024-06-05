use axum::Router;
use buenzlimarks_server::{
    config::Config, docs::docs_handler, frontend::frontend_handler, router::api_router,
};
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
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
        .fallback(frontend_handler)
        .layer(CompressionLayer::new());

    // run it
    let listener = TcpListener::bind("0.0.0.0:4000").await.unwrap();
    tracing::info!("listening on http://localhost:4000");
    axum::serve(listener, router).await.unwrap();
}
