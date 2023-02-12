use serde::{Deserialize, Serialize};

use super::{id::Id, widget::Widget};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: Id<Self>,
    pub name: String,
    pub url: String,
    pub widget_id: Id<Widget>,
}
