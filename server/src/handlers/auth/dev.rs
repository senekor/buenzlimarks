use axum::{
    extract::{FromRequestParts, Path, State},
    headers::{self, authorization::Bearer},
    http::{request::Parts, StatusCode},
    routing::get,
    Extension, Router, TypedHeader,
};

use crate::{
    db::{error::DbError, DB},
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

#[tracing::instrument(skip(db))]
pub async fn login(
    Path(user_id): Path<Id<User>>,
    State(db): State<DB>,
) -> Result<String, StatusCode> {
    // tracing::debug!("present user: {present_user:?}");
    let user = match db.get_user(&user_id) {
        Ok(present_user) => {
            tracing::debug!("user was already present: {present_user:?}");
            present_user
        }
        Err(DbError::NotFound) => {
            tracing::debug!(
                "user with id {:?} was not found, attempting to insert",
                &user_id
            );
            match db.insert_user(User::with_id_as_name(&user_id)) {
                Ok(inserted_user) => {
                    tracing::debug!("successfully inserted new user: {:?}", &inserted_user);
                    inserted_user
                }
                Err(e) => {
                    tracing::error!("DB failed to insert error: {e:?}");
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
        Err(e) => {
            tracing::error!("db returned garbage error while fetching user: {e:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    tracing::debug!("login succeeded for user {user:?}");
    Ok(user.id.into())
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
