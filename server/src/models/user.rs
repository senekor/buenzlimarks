use serde::{Deserialize, Serialize};

use super::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Id<Self>,
    pub name: Option<String>,
}

#[cfg(debug_assertions)]
pub static DEV_USER_ID_STR: &str = "buenzli";

pub fn dev_user_id() -> Id<User> {
    Id::dev_user_id()
}

impl User {
    #[cfg(debug_assertions)]
    /// returns the default development user
    pub fn dev() -> Self {
        Self {
            id: dev_user_id(),
            name: Some("Bünzli".into()),
        }
    }

    #[cfg(debug_assertions)]
    /// returns a new user without a name
    pub fn with_id_as_name(user_id: &Id<User>) -> Self {
        Self {
            id: user_id.clone(),
            name: Some(user_id.to_string()),
        }
    }
}
