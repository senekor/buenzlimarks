use leptos::*;
use models::{Bookmark as BookmarkType, Id};

use crate::{
    api::use_entity,
    components::FlexSpace,
    icons::{PencilSquareIcon, XMarkIcon},
};

#[component]
pub fn Bookmark(
    cx: Scope,
    bookmark: BookmarkType,
    set_bookmark_form: WriteSignal<BookmarkType>,
    delete_bookmark: Action<Id<BookmarkType>, bool>,
) -> impl IntoView {
    let original_bookmark = store_value(cx, bookmark);
    let id = Signal::derive(cx, move || original_bookmark().id);

    let bookmark_resource = use_entity::<BookmarkType>(cx, id.get_untracked());
    let bookmark = create_memo(cx, move |_| match bookmark_resource.read(cx).flatten() {
        Some(b) => b,
        None => original_bookmark(),
    });

    view! { cx,
        <div class="flex w-full gap-1">
            <FlexSpace />
            <a
                class="text-2xl text-orange-200 hover:text-orange-400 underline"
                href=move || bookmark().url
            >
                { move || bookmark().name }
            </a>
            <FlexSpace />
            <button
                class="w-6 ml-2"
                on:click=move |_| set_bookmark_form(bookmark())
            >
                <PencilSquareIcon />
            </button>
            <button
                class="w-6"
                on:click=move |_| delete_bookmark.dispatch(bookmark().id)
            >
                <XMarkIcon />
            </button>
        </div>
    }
}
