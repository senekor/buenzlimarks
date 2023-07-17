use leptos::*;
use models::{Bookmark as BookmarkType, Widget as WidgetType};

use crate::{
    api::{create_delete_entity, create_submit_entity, use_entity, use_filtered_entities},
    components::{Bookmark, FlexSpace},
    edit_mode::use_edit_mode,
    icons::{PencilSquareIcon, XMarkIcon},
};

#[component]
pub fn Widget(cx: Scope, widget: WidgetType) -> impl IntoView {
    let id = store_value(cx, widget.id.clone());
    let widget = use_entity(cx, widget);

    let name = Signal::derive(cx, move || widget().name);
    let page_id = Signal::derive(cx, move || widget().page_id);

    let bookmarks = use_filtered_entities::<BookmarkType>(cx, id());

    let submit_widget = create_submit_entity::<WidgetType>(cx);
    let delete_widget = create_delete_entity::<WidgetType>(cx);
    let delete_bookmark = create_delete_entity::<BookmarkType>(cx);

    let (name_form, set_name_form) = create_signal::<Option<String>>(cx, None);

    let edit_mode = use_edit_mode(cx).read();

    view! { cx,
        <div class="bg-slate-700 flex flex-col p-4 rounded-lg">
            <div class="flex flex-row gap-2 items-center pb-2">
                <FlexSpace />
                <h2 class="text-3xl" hidden=move || name_form().is_some() >{ name }</h2>
                <input
                    class="bg-slate-600 p-1 px-2 rounded text-lg"
                    hidden=move || name_form().is_none()
                    prop:value=name_form
                    on:input=move |ev| { set_name_form(Some(event_target_value(&ev))); }
                    on:keydown=move |ev| {
                        if &ev.key() == "Enter" {
                            submit_widget.dispatch(WidgetType {
                                id: id(),
                                name: name_form.get_untracked().unwrap_or_default(),
                                page_id: page_id.get_untracked(),
                            });
                            set_name_form(None);
                        }
                    }
                />
                <FlexSpace />
                <Show
                    when=edit_mode
                    fallback=|_| ()
                >
                    <div class="flex flex-row gap-1 items-center">
                        <button on:click=move |_| set_name_form(Some(name()))>
                            <PencilSquareIcon />
                        </button>
                        <button on:click=move |_| delete_widget.dispatch(id())>
                            <XMarkIcon />
                        </button>
                    </div>
                </Show>
            </div>
            <For
                each=move || bookmarks.read(cx).unwrap_or_default()
                key=|bookmark| bookmark.id.clone()
                view=move |cx, bookmark| {
                    let bookmark = store_value(cx, bookmark);
                    view! { cx,
                        <Bookmark bookmark=bookmark() delete_bookmark />
                    }
                }
            />
        </div>
    }
}
