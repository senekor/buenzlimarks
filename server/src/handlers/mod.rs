use axum::Router;

use crate::db::DB;

mod auth;
mod bookmarks;
mod users;

pub fn routes(db: DB) -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
        .nest("/bookmarks", bookmarks::routes())
        .with_state(db)
        .layer(auth::extension())
}
