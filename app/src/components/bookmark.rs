use leptos::*;
use models::Bookmark as BookmarkType;

use crate::{
    components::{BookmarkForm, Dialog, FlexSpace},
    edit_mode::use_edit_mode,
    icons::{PencilSquareIcon, XMarkIcon},
    state::use_store,
    state::Action,
};

#[component]
pub fn Bookmark(bookmark: BookmarkType) -> impl IntoView {
    let store = use_store();

    let bookmark = store_value(bookmark);

    let edit_mode = use_edit_mode().read();
    let no_edit_mode = Signal::derive(move || !edit_mode());

    let (form_open, set_form_open) = create_signal(false);
    let on_close = move || set_form_open(false);

    view! {
        <div class="flex w-full gap-1">
            <FlexSpace />
            <a
                class="text-orange-200 hover:text-orange-400 underline"
                href=bookmark().url
            >
                { bookmark().name }
            </a>
            <FlexSpace />
            <button
                hidden=no_edit_mode
                class="w-6 ml-2"
                on:click=move |_| set_form_open(true)
            >
                <PencilSquareIcon />
            </button>
            <button
                hidden=no_edit_mode
                class="w-6"
                on:click=move |_| store.dispatch(Action::DeleteBookmark(bookmark()))
            >
                <XMarkIcon />
            </button>
        </div>
        <Show when=form_open fallback=|| () >
            <Dialog on_close >
                <BookmarkForm on_close prev_bookmark=bookmark() />
            </Dialog>
        </Show>

    }
}
