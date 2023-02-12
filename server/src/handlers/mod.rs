use axum::Router;

use crate::db::DB;

mod auth;
mod bookmarks;

pub fn routes(db: DB) -> Router {
    Router::new()
        // .nest("/auth", auth::routes())
        .nest("/bookmarks", bookmarks::routes())
        .with_state(db)
    // .layer(Extension(auth::jwt_key()))
}
