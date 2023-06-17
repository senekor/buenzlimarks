use serde::{Deserialize, Serialize};

use super::{id::Id, page::Page};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Widget {
    pub id: Id<Self>,
    pub name: String,
    pub page_id: Id<Page>,
}
