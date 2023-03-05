use serde::{Deserialize, Serialize};

use super::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Id<Self>,
    pub name: Option<String>,
}

#[cfg(debug_assertions)]
pub static DEV_USER_ID_STR: &str = "dev_user";

pub fn dev_user_id() -> Id<User> {
    Id::dev_user_id()
}

impl User {
    /// returns the default development user
    pub fn dev() -> Self {
        Self {
            id: dev_user_id(),
            name: Some("Hackerman".into()),
        }
    }

    /// returns a new user without a name
    pub fn anonymous(user_id: &Id<User>) -> Self {
        Self {
            id: user_id.clone(),
            name: None,
        }
    }
}
