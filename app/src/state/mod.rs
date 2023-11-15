mod action;
mod middleware;
mod reducer;
mod store;

use std::collections::HashMap;

use models::{Bookmark, Id, Page, Settings, Widget};

use self::middleware::PreMiddlewareAction;
pub use self::store::{provide_store, use_store};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    settings: Settings,
    pages: Vec<Page>,
    widgets: HashMap<Id<Page>, Vec<Widget>>,
    bookmarks: HashMap<Id<Widget>, Vec<Bookmark>>,
}

// Users of the store only need to know about the pre-middleware actions.
// The fact that middleware is run is an implementation detail.
pub type Action = PreMiddlewareAction;
