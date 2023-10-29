use leptos::*;
use models::{Bookmark as BookmarkType, Id};

use crate::{
    api::use_entity,
    components::{BookmarkForm, Dialog, FlexSpace},
    edit_mode::use_edit_mode,
    icons::{PencilSquareIcon, XMarkIcon},
};

#[component]
pub fn Bookmark(
    bookmark: BookmarkType,
    delete_bookmark: Action<Id<BookmarkType>, bool>,
) -> impl IntoView {
    // let id = store_value( bookmark.id.clone());
    let bookmark = use_entity(bookmark);

    let edit_mode = use_edit_mode().read();
    let no_edit_mode = Signal::derive(move || !edit_mode());

    let (form_open, set_form_open) = create_signal(false);
    let on_close = move || set_form_open(false);

    view! {
        <div class="flex w-full gap-1">
            <FlexSpace />
            <a
                class="text-orange-200 hover:text-orange-400 underline"
                href=move || bookmark().url
            >
                { move || bookmark().name }
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
                on:click=move |_| delete_bookmark.dispatch(bookmark().id)
            >
                <XMarkIcon />
            </button>
        </div>
        <Show when=form_open fallback=|| () >
            <Dialog on_close >
                <BookmarkForm on_close prev_bookmark=bookmark.get_untracked() />
            </Dialog>
        </Show>

    }
}
