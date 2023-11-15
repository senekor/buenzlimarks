use models::{Bookmark, Page, Widget};

use super::State;

#[derive(Debug)]
pub enum Action {
    Overwrite(State),
    AddPage(Page),
    AddWidget(Widget),
    AddBookmark(Bookmark),
    RemovePage(Page),
    RemoveWidget(Widget),
    RemoveBookmark(Bookmark),
}
