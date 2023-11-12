use serde::{Deserialize, Serialize};

use super::{Id, Page};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Widget {
    pub id: Id<Self>,
    pub name: String,
    pub page_id: Id<Page>,
}
