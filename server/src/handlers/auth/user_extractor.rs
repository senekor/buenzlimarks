use axum::{
    extract::FromRequestParts,
    headers::{self, authorization::Bearer},
    http::{request::Parts, StatusCode},
    TypedHeader,
};

use crate::{models::User, state::AppState};

type BearerAuth = TypedHeader<headers::Authorization<Bearer>>;

#[axum::async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = StatusCode;

    async fn from_request_parts(
        req: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let bearer = BearerAuth::from_request_parts(req, state)
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
        let token = bearer.0.token();

        super::user_from(token, state).ok_or(StatusCode::UNAUTHORIZED)
    }
}
