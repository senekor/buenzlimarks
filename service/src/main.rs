use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Extension, Router};
use derive_deref::{Deref, DerefMut};
use std::{collections::HashMap, io, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

mod models;
use models::Bookmark;

mod handlers;
use handlers::api_routes;

async fn internal_err(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

fn frontend_routes() -> Router {
    let app_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("app/dist");

    Router::new().fallback(get_service(ServeDir::new(app_dir)).handle_error(internal_err))
}

#[derive(Debug, Default, Deref, DerefMut)]
pub struct InMemDB(HashMap<String, Vec<Bookmark>>);

async fn insert_default_data(db: Arc<Mutex<InMemDB>>) {
    let mut acq_db = db.lock().await;

    acq_db.insert(
        "remo".to_string(),
        vec![Bookmark::new(
            "Tasks",
            "https://github.com/users/remlse/projects/1/views/2",
        )],
    );
    acq_db.insert(
        "silvia".to_string(),
        vec![Bookmark::new(
            "Tasks",
            "https://github.com/users/remlse/projects/1/views/4",
        )],
    );
    acq_db.insert(
        "harald".to_string(),
        vec![
            Bookmark::new(
                "Requirements",
                "https://github.com/users/remlse/projects/1/views/6",
            ),
            Bookmark::new(
                "Prioritization",
                "https://github.com/users/remlse/projects/1/views/7",
            ),
        ],
    );
}

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(InMemDB::default()));

    insert_default_data(db.clone()).await;

    let http_service = Router::new()
        .nest("/api", api_routes())
        .layer(Extension(db))
        .merge(frontend_routes());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(http_service.into_make_service())
        .await
        .unwrap();
}
