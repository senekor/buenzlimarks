use axum::Router;

use crate::db::{new_db, DB};

// mod auth;
mod bookmarks;

pub fn routes() -> Router {
    let db = new_db();

    Router::<DB>::new()
        // .nest("/auth", auth::routes())
        .nest("/bookmarks", bookmarks::routes())
        .with_state(db)
    // .layer(Extension(auth::jwt_key()))
}
