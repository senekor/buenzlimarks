use std::{fmt::Debug, hash::Hash};

use serde::{Deserialize, Serialize};

use super::{Bookmark, Id, Page, Widget};

type EqChild<T> = fn(&Id<T>, &<T as Entity>::Child) -> bool;

pub trait Entity:
    Debug + Clone + PartialEq + Hash + 'static + Serialize + for<'a> Deserialize<'a>
{
    type Parent: Debug + Entity;
    type Child: Debug + Entity;
    const DATA: EntityData;
    fn get_id(&self) -> &Id<Self>;
    fn get_parent_id(&self) -> Option<&Id<Self::Parent>>;
    fn eq_child() -> Option<EqChild<Self>>;
}

impl Entity for Page {
    type Parent = Page;
    type Child = Widget;
    const DATA: EntityData = EntityData::Page;

    fn get_id(&self) -> &Id<Self> {
        &self.id
    }

    fn get_parent_id(&self) -> Option<&Id<Self::Parent>> {
        None
    }

    fn eq_child() -> Option<fn(&Id<Page>, &Widget) -> bool> {
        fn impl_eq_child(id: &Id<Page>, widget: &Widget) -> bool {
            id == &widget.page_id
        }
        Some(impl_eq_child)
    }
}

impl Entity for Widget {
    type Parent = Page;
    type Child = Bookmark;
    const DATA: EntityData = EntityData::Widget;

    fn get_id(&self) -> &Id<Self> {
        &self.id
    }

    fn get_parent_id(&self) -> Option<&Id<Self::Parent>> {
        Some(&self.page_id)
    }

    fn eq_child() -> Option<fn(&Id<Widget>, &Bookmark) -> bool> {
        fn impl_eq_child(id: &Id<Widget>, bookmark: &Bookmark) -> bool {
            id == &bookmark.widget_id
        }
        Some(impl_eq_child)
    }
}

impl Entity for Bookmark {
    type Parent = Widget;
    type Child = Bookmark;
    const DATA: EntityData = EntityData::Bookmark;

    fn get_id(&self) -> &Id<Self> {
        &self.id
    }

    fn get_parent_id(&self) -> Option<&Id<Self::Parent>> {
        Some(&self.widget_id)
    }

    fn eq_child() -> Option<fn(&Id<Self>, &Self::Child) -> bool> {
        None
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
