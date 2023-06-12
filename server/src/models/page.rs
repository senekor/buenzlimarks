use serde::{Deserialize, Serialize};

use super::id::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    pub id: Id<Self>,
    pub name: String,
}
