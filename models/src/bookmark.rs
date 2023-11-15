use serde::{Deserialize, Serialize};

use super::{Id, Widget};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Bookmark {
    pub id: Id<Self>,
    pub name: String,
    pub url: String,
    pub widget_id: Id<Widget>,
}

pub fn sanitize_bookmark(bookmark: &mut Bookmark) {
    if !bookmark.url.contains("://") {
        bookmark.url = format!("https://{}", bookmark.url);
    };
}
