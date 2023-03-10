use axum::{
    extract::{FromRequestParts, Path, State},
    headers::{self, authorization::Bearer},
    http::{request::Parts, StatusCode},
    routing::get,
    Extension, Router, TypedHeader,
};

use crate::{
    db::DB,
    models::{id::Id, user::User},
};

type BearerAuth = TypedHeader<headers::Authorization<Bearer>>;

#[axum::async_trait]
impl FromRequestParts<DB> for User {
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(req: &mut Parts, db: &DB) -> Result<Self, Self::Rejection> {
        let Ok(bearer) = BearerAuth::from_request_parts(req, db).await else {
            return Err(StatusCode::UNAUTHORIZED);
        };
        let token = bearer.0.token();
        let user_id = Id::<User>::from(token);

        db.get_user(&user_id).map_err(|_| StatusCode::UNAUTHORIZED)
    }
}

async fn login(Path(user_id): Path<Id<User>>, State(db): State<DB>) -> (StatusCode, String) {
    match db
        .get_user(&user_id)
        .or_else(|_| db.insert_user(User::anonymous(&user_id)))
    {
        Ok(_) => (StatusCode::OK, user_id.into()),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::new()),
    }
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
