use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::models::{bookmark::Bookmark, page::Page, widget::Widget};

pub trait DbEntity: Debug + Serialize + for<'a> Deserialize<'a> {
    fn plural() -> &'static str;
}

impl DbEntity for Page {
    fn plural() -> &'static str {
        "pages"
    }
}

impl DbEntity for Widget {
    fn plural() -> &'static str {
        "widgets"
    }
}

impl DbEntity for Bookmark {
    fn plural() -> &'static str {
        "bookmarks"
    }
}
