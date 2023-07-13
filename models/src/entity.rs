use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::{Bookmark, Id, Page, Widget};

pub trait Entity:
    Debug + Clone + PartialEq + 'static + Serialize + for<'a> Deserialize<'a>
{
    type Parent: Debug + Entity;
    const DATA: EntityData;
    fn get_id(&self) -> &Id<Self>;
    fn get_parent_id(&self) -> Option<&Id<Self::Parent>>;
}

impl Entity for Page {
    type Parent = Page;
    const DATA: EntityData = EntityData::Page;
    fn get_id(&self) -> &Id<Self> {
        &self.id
    }

    fn get_parent_id(&self) -> Option<&Id<Self::Parent>> {
        None
    }
}

impl Entity for Widget {
    type Parent = Page;
    const DATA: EntityData = EntityData::Widget;
    fn get_id(&self) -> &Id<Self> {
        &self.id
    }

    fn get_parent_id(&self) -> Option<&Id<Self::Parent>> {
        Some(&self.page_id)
    }
}

impl Entity for Bookmark {
    type Parent = Widget;
    const DATA: EntityData = EntityData::Bookmark;
    fn get_id(&self) -> &Id<Self> {
        &self.id
    }

    fn get_parent_id(&self) -> Option<&Id<Self::Parent>> {
        Some(&self.widget_id)
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
