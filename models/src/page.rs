use serde::{Deserialize, Serialize};

use super::Id;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Page {
    pub id: Id<Self>,
    pub name: String,
}
