use axum::Router;

mod bookmarks;

pub fn api_routes() -> Router {
    Router::new().nest("/bookmarks", bookmarks::routes())
}
