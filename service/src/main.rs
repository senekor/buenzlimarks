use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Extension, Json, Router,
};
use derive_deref::{Deref, DerefMut};
use std::{collections::HashMap, io, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Bookmark {
    id: u64,
    name: String,
    url: String,
}

impl Bookmark {
    fn new(name: &str, url: &str) -> Self {
        Self {
            id: rand::random(),
            name: name.to_string(),
            url: url.to_string(),
        }
    }

    fn randomize_id(&mut self) {
        self.id = rand::random()
    }

    fn ensure_protocol_prefix(&mut self) {
        if !self.url.starts_with("http") {
            self.url = format!("https://{}", self.url);
        }
    }
}

#[derive(Debug, Default, Deref, DerefMut)]
struct InMemDB(HashMap<String, Vec<Bookmark>>);

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
        .route("/api/:name", get(get_bookmarks).post(add_bookmark))
        .layer(Extension(db))
        .merge(frontend_router());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(http_service.into_make_service())
        .await
        .unwrap();
}

async fn get_bookmarks(
    Extension(db): Extension<Arc<Mutex<InMemDB>>>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let bookmarks = db
        .lock()
        .await
        .get(&name.to_lowercase())
        .map(Clone::clone)
        .unwrap_or_default();
    Json(bookmarks)
}

async fn add_bookmark(
    Extension(db): Extension<Arc<Mutex<InMemDB>>>,
    Path(name): Path<String>,
    Json(mut bookmark): Json<Bookmark>,
) -> impl IntoResponse {
    let mut db = db.lock().await;
    let bookmarks = db.entry(name.to_lowercase()).or_default();
    bookmark.randomize_id();
    bookmark.ensure_protocol_prefix();
    bookmarks.push(bookmark);
    StatusCode::CREATED
}
