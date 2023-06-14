use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    db,
    handlers::{
        auth::{self, login},
        bookmarks::{create_bookmark, delete_bookmark, get_bookmarks, update_bookmark},
        pages::{create_page, get_page, get_pages},
        users::whoami,
        widgets::{create_widget, get_widget, get_widgets},
    },
};

pub fn api_router() -> Router {
    let db = db::get();

    Router::new()
        // POST - create
        .route("/pages", post(create_page))
        .route("/widgets", post(create_widget))
        .route("/bookmarks", post(create_bookmark))
        //
        // GET - read
        .route("/pages/:page_id", get(get_page))
        .route("/pages", get(get_pages))
        .route("/widgets/:widget_id", get(get_widget))
        .route("/widgets", get(get_widgets))
        .route("/bookmarks", get(get_bookmarks))
        //
        // PUT - update
        .route("/bookmarks", put(update_bookmark))
        //
        // DELETE - delete
        .route("/bookmarks/:bookmark_id", delete(delete_bookmark))
        //
        // authentication
        .route("/auth/login/:user_id", get(login))
        .route("/users/me", get(whoami))
        //
        // shared state
        .with_state(db)
        .layer(auth::extension())
}
