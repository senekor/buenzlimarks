use leptos::{prelude::*, task::spawn_local};
use models::{Bookmark, Id, Page, Settings, Widget};

use super::{
    action::Action,
    middleware::{self, PreMiddlewareAction},
    reducer::reduce,
    State,
};

#[derive(Debug, Clone, Copy)]
pub struct Store {
    state: RwSignal<State>,
}

/// Basically a wrapper around [leptos::RwSignal]. The only difference is that
/// updates have to go through the `dispatch` method.
impl Store {
    fn new() -> Self {
        let store = Self {
            state: RwSignal::new(Default::default()),
        };
        // Load initial state
        store.dispatch(PreMiddlewareAction::Reload);

        store
    }

    pub fn dispatch(self, action: PreMiddlewareAction) {
        spawn_local(async move {
            let action = middleware::process(action).await;
            self.dispatch_without_middleware(action);
        });
    }

    fn dispatch_without_middleware(&self, action: Action) {
        let new_state = self
            .state
            .with_untracked(|state| reduce(state.clone(), action));
        if self.state.with_untracked(|state| &new_state != state) {
            self.state.set(new_state);
        }
    }

    pub fn _settings(self) -> Signal<Settings> {
        Memo::new(move |_| self.state.with(|s| s.settings.clone())).into()
    }

    pub fn pages(self) -> Signal<Vec<Page>> {
        Memo::new(move |_| self.state.with(|s| s.pages.clone())).into()
    }

    pub fn widgets(self) -> Signal<Vec<Widget>> {
        Memo::new(move |_| {
            self.state
                .with(|s| s.widgets.values().flatten().cloned().collect())
        })
        .into()
    }

    pub fn widgets_by(self, page_id: Id<Page>) -> Signal<Vec<Widget>> {
        Memo::new(move |_| {
            self.state
                .with(|s| s.widgets.get(&page_id).cloned().unwrap_or_default())
        })
        .into()
    }

    pub fn _bookmarks(self) -> Signal<Vec<Bookmark>> {
        Memo::new(move |_| {
            self.state
                .with(|s| s.bookmarks.values().flatten().cloned().collect())
        })
        .into()
    }

    pub fn bookmarks_by(self, widget_id: Id<Widget>) -> Signal<Vec<Bookmark>> {
        Memo::new(move |_| {
            self.state
                .with(|s| s.bookmarks.get(&widget_id).cloned().unwrap_or_default())
        })
        .into()
    }
}

pub fn provide_store() {
    provide_context(Store::new());
}

pub fn use_store() -> Store {
    use_context().expect("should find store context")
}
