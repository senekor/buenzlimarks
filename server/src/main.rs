use axum::{
    routing::{delete, get, post, put},
    Router,
};
use lib::{
    db, frontend,
    handlers::{
        auth::{self, login},
        bookmarks::{create_bookmark, delete_bookmark, get_bookmarks, update_bookmark},
        pages::{create_page, get_page, get_pages},
        users::whoami,
        widgets::{create_widget, get_widget, get_widgets},
    },
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let db = db::get();

    let api_router = Router::new()
        .route("/auth/login/:user_id", get(login))
        .route("/users/me", get(whoami))
        //
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
        .with_state(db)
        .layer(auth::extension());

    let router = Router::new()
        .nest("/api", api_router)
        .merge(frontend::frontend_routes());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
