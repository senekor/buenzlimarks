use axum::Json;

use crate::models::user::User;

pub async fn whoami(user: User) -> Json<User> {
    Json(user)
}
