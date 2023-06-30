use std::{convert::Infallible, fmt::Debug};

use serde::{Deserialize, Serialize};

use super::{Bookmark, Id, Page, Widget};

pub trait Entity:
    Debug + Clone + PartialEq + 'static + Serialize + for<'a> Deserialize<'a>
{
    type Parent: Debug;
    const DATA: EntityData;
    fn get_id(&self) -> &Id<Self>;
}

impl Entity for Page {
    type Parent = Infallible;
    const DATA: EntityData = EntityData::Page;
    fn get_id(&self) -> &Id<Self> {
        &self.id
    }
}

impl Entity for Widget {
    type Parent = Page;
    const DATA: EntityData = EntityData::Widget;
    fn get_id(&self) -> &Id<Self> {
        &self.id
    }
}

impl Entity for Bookmark {
    type Parent = Widget;
    const DATA: EntityData = EntityData::Bookmark;
    fn get_id(&self) -> &Id<Self> {
        &self.id
    }
}

pub enum EntityData {
    Page,
    Widget,
    Bookmark,
}

impl EntityData {
    pub const fn plural(&self) -> &'static str {
        use EntityData::*;
        match self {
            Page => "pages",
            Widget => "widgets",
            Bookmark => "bookmarks",
        }
    }
    pub const fn parent_id(&self) -> &'static str {
        use EntityData::*;
        match self {
            Page => panic!("page doesn't have a parent entity"),
            Widget => "page_id",
            Bookmark => "widget_id",
        }
    }
}
