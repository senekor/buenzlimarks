use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::models::{Bookmark, Id, Page, Widget};

pub trait DbEntity: Debug + Serialize + for<'a> Deserialize<'a> {
    fn get_id(&self) -> &Id<Self>;
    fn plural() -> &'static str;
}

impl DbEntity for Page {
    fn get_id(&self) -> &Id<Self> {
        &self.id
    }
    fn plural() -> &'static str {
        "pages"
    }
}

impl DbEntity for Widget {
    fn get_id(&self) -> &Id<Self> {
        &self.id
    }
    fn plural() -> &'static str {
        "widgets"
    }
}

impl DbEntity for Bookmark {
    fn get_id(&self) -> &Id<Self> {
        &self.id
    }
    fn plural() -> &'static str {
        "bookmarks"
    }
}
