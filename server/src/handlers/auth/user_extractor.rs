use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::{
    headers::{self, authorization::Bearer},
    TypedHeader,
};
use models::User;

use crate::state::AppState;

type BearerAuth = TypedHeader<headers::Authorization<Bearer>>;

#[axum::async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = StatusCode;

    #[tracing::instrument]
    async fn from_request_parts(
        req: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let bearer = BearerAuth::from_request_parts(req, state)
            .await
            .map_err(|_| {
                tracing::debug!("missing bearer auth");
                StatusCode::UNAUTHORIZED
            })?;
        let token = bearer.0.token();

        super::user_from(token, state).ok_or_else(|| {
            tracing::debug!("invalid token");
            StatusCode::UNAUTHORIZED
        })
    }
}
