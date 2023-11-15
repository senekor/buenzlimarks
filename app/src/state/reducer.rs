use super::{action::Action, State};

pub(super) fn reduce(mut state: State, action: Action) -> State {
    match action {
        Action::Overwrite(state) => state,
        Action::AddPage(page) => {
            state = reduce(state, Action::RemovePage(page.clone()));
            state.pages.push(page);
            state
        }
        Action::AddWidget(widget) => {
            state = reduce(state, Action::RemoveWidget(widget.clone()));
            state
                .widgets
                .entry(widget.page_id.clone())
                .or_default()
                .push(widget);
            state
        }
        Action::AddBookmark(bookmark) => {
            state = reduce(state, Action::RemoveBookmark(bookmark.clone()));
            state
                .bookmarks
                .entry(bookmark.widget_id.clone())
                .or_default()
                .push(bookmark);
            state
        }
        Action::RemovePage(page) => {
            if let Some(idx) = state.pages.iter().position(|p| p.id == page.id) {
                state.pages.remove(idx);
            }
            state
        }
        Action::RemoveWidget(widget) => {
            let Some(widgets) = state.widgets.get_mut(&widget.page_id) else {
                return state;
            };
            if let Some(idx) = widgets.iter().position(|w| w.id == widget.id) {
                widgets.remove(idx);
            }
            state
        }
        Action::RemoveBookmark(bookmark) => {
            let Some(bookmarks) = state.bookmarks.get_mut(&bookmark.widget_id) else {
                return state;
            };
            if let Some(idx) = bookmarks.iter().position(|b| b.id == bookmark.id) {
                bookmarks.remove(idx);
            }
            state
        }
    }
}
