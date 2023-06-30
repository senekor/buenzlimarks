use leptos::*;
use models::Bookmark as BookmarkType;

use crate::api::use_entity;

#[component]
pub fn Bookmark(cx: Scope, bookmark: BookmarkType) -> impl IntoView {
    let original_bookmark = store_value(cx, bookmark);
    let id = Signal::derive(cx, move || original_bookmark().id);

    let bookmark_resource = use_entity::<BookmarkType>(cx, id.get_untracked());
    let bookmark = create_memo(cx, move |_| match bookmark_resource.read(cx).flatten() {
        Some(b) => b,
        None => original_bookmark(),
    });

    view! { cx,
        <a
            class="text-2xl text-orange-200 hover:text-orange-400 underline"
            href=move || bookmark().url
        >
            { move || bookmark().name }
        </a>
    }
}
