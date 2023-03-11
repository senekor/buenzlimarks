use axum::{
    routing::{delete, get, post},
    Router,
};
use lib::{
    db, frontend,
    handlers::{
        auth::{self, login},
        bookmarks::{create_bookmark, delete_bookmark, get_bookmarks},
        pages::create_page,
        users::whoami,
    },
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let db = db::get();

    let router = Router::new()
        .route("/api/auth/login/:user_id", get(login))
        .route("/api/users/me", get(whoami))
        .route("/api/pages", post(create_page))
        .route("/api/bookmarks", get(get_bookmarks))
        .route("/api/bookmarks", post(create_bookmark))
        .route("/api/bookmarks/:bookmark_id", delete(delete_bookmark))
        .with_state(db)
        .layer(auth::extension())
        .merge(frontend::frontend_routes());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
