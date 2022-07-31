use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Extension, Json, Router,
};
use tokio::sync::Mutex;

use crate::{models::Bookmark, InMemDB};

impl Bookmark {
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            id: rand::random::<u64>().to_string(),
            name: name.to_string(),
            url: url.to_string(),
        }
    }

    fn randomize_id(&mut self) {
        self.id = rand::random::<u64>().to_string()
    }

    fn ensure_protocol_prefix(&mut self) {
        if !self.url.starts_with("http") {
            self.url = format!("https://{}", self.url);
        }
    }
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
    StatusCode::OK
}

async fn update_bookmark(
    Extension(db): Extension<Arc<Mutex<InMemDB>>>,
    Path(name): Path<String>,
    Json(mut bookmark): Json<Bookmark>,
) -> impl IntoResponse {
    let mut db = db.lock().await;
    let bookmarks = db.entry(name.to_lowercase()).or_default();
    bookmark.ensure_protocol_prefix();
    if let Some((idx, _)) = bookmarks
        .iter()
        .enumerate()
        .find(|(_, bm)| bm.id == bookmark.id)
    {
        bookmarks[idx] = bookmark;
        return StatusCode::OK;
    }
    StatusCode::NOT_FOUND
}

async fn delete_bookmark(
    Extension(db): Extension<Arc<Mutex<InMemDB>>>,
    Path(name): Path<String>,
    Json(bookmark): Json<Bookmark>,
) -> impl IntoResponse {
    let mut db = db.lock().await;
    let bookmarks = db.entry(name.to_lowercase()).or_default();
    if let Some((idx, _)) = bookmarks
        .iter()
        .enumerate()
        .find(|(_, bm)| bm.id == bookmark.id)
    {
        bookmarks.remove(idx);
        return StatusCode::OK;
    }
    StatusCode::NOT_FOUND
}

pub fn routes() -> Router {
    Router::new().route(
        "/:name",
        get(get_bookmarks)
            .post(add_bookmark)
            .put(update_bookmark)
            .delete(delete_bookmark),
    )
}
