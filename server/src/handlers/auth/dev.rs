use axum::{
    extract::{FromRequestParts, Path, State, TypedHeader},
    headers,
    http::{header, request::Parts, HeaderMap, StatusCode},
    routing::get,
    Extension, Router,
};

use crate::{
    db::DB,
    models::{id::Id, user::User},
};

use super::COOKIE_NAME;

type CookieHeader = TypedHeader<headers::Cookie>;

#[axum::async_trait]
impl FromRequestParts<DB> for User {
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(req: &mut Parts, db: &DB) -> Result<Self, Self::Rejection> {
        let Ok(cookies) = CookieHeader::from_request_parts(req, db).await else {
            return Ok(User::dev());
        };

        let Some(user_id) = cookies.get(COOKIE_NAME).map(Id::<User>::from) else {
            return Ok(User::dev());
        };

        db.get_user(&user_id).map_err(|_| StatusCode::UNAUTHORIZED)
    }
}

async fn login(Path(user_id): Path<Id<User>>, State(db): State<DB>) -> (StatusCode, HeaderMap) {
    if db
        .get_user(&user_id)
        .or_else(|_| db.insert_user(User::anonymous(&user_id)))
        .is_err()
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, HeaderMap::new());
    };

    let cookie = format!("{COOKIE_NAME}={user_id}; SameSite=Lax; Path=/");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        "Set-Cookie".parse().unwrap(),
    );
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    (StatusCode::OK, headers)
}

pub fn routes() -> Router<DB> {
    Router::<DB>::new().route("/login/:user_id", get(login))
}

/// This is a no-op. However, an identically named function in `auth::prod`
/// provides the key needed to decrypt the jwt tokens to every handler.
/// This enables them to authenticate incoming requests.
pub fn extension() -> Extension<()> {
    Extension(())
}
