use serde::{Deserialize, Serialize};
use strum_macros::Display;

use super::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum AuthProvider {
    #[serde(rename = "dev")]
    #[strum(serialize = "dev")]
    Dev,
    #[serde(rename = "github")]
    #[strum(serialize = "github")]
    GitHub,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: Id<Self>,
    pub provider: AuthProvider,
}
