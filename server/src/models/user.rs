use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: Option<String>,
}

pub const DEV_USER: &str = "dev_user";

impl User {
    /// returns the default development user
    pub fn dev() -> Self {
        Self {
            id: DEV_USER.into(),
            name: Some("Hackerman".into()),
        }
    }
}
