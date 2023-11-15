use serde::{Deserialize, Serialize};

use super::Id;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Page {
    pub id: Id<Self>,
    pub name: String,
}
