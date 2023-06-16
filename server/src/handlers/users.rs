use axum::Json;

use crate::models::user::User;

#[tracing::instrument]
pub async fn whoami(user: User) -> Json<User> {
    Json(user)
}
