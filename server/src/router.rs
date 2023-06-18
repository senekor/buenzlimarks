use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    config::Config,
    handlers::{
        auth,
        bookmarks::{
            create_bookmark, delete_bookmark, get_bookmark, get_bookmarks, update_bookmark,
        },
        pages::{create_page, delete_page, get_page, get_pages, update_page},
        settings::settings,
        widgets::{create_widget, delete_widget, get_widget, get_widgets, update_widget},
    },
    state::AppState,
};

pub fn api_router(config: &Config) -> Router {
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
        .route("/bookmarks/:bookmark_id", get(get_bookmark))
        .route("/bookmarks", get(get_bookmarks))
        //
        // PUT - update
        .route("/pages", put(update_page))
        .route("/widgets", put(update_widget))
        .route("/bookmarks", put(update_bookmark))
        //
        // DELETE - delete
        .route("/pages/:page_id", delete(delete_page))
        .route("/widgets/:widget_id", delete(delete_widget))
        .route("/bookmarks/:bookmark_id", delete(delete_bookmark))
        //
        // authentication
        .nest("/auth", auth::routes())
        .route("/settings", get(settings))
        //
        // shared state
        .with_state(AppState::new(config))
}
