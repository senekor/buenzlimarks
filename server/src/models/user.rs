use serde::{Deserialize, Serialize};

use super::id::Id;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Id<Self>,
    pub name: Option<String>,
}

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
}
