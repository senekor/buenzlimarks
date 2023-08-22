use leptos::*;
use models::{Bookmark as BookmarkType, Widget as WidgetType};

use crate::{
    api::{create_delete_entity, use_entity, use_filtered_entities},
    components::{Bookmark, ConfirmationDialog, Dialog, FlexSpace, WidgetForm},
    edit_mode::use_edit_mode,
    icons::{PencilSquareIcon, XMarkIcon},
};

#[component]
pub fn Widget(cx: Scope, widget: WidgetType) -> impl IntoView {
    let id = store_value(cx, widget.id.clone());
    let widget = use_entity(cx, widget);

    let name = Signal::derive(cx, move || widget().name);

    let bookmarks = use_filtered_entities::<BookmarkType>(cx, id());

    let delete_widget = create_delete_entity::<WidgetType>(cx);
    let delete_bookmark = create_delete_entity::<BookmarkType>(cx);

    let edit_mode = use_edit_mode(cx).read();

    let (form_open, set_form_open) = create_signal(cx, false);
    let on_form_close = move || set_form_open(false);

    let (delete_open, set_delete_open) = create_signal(cx, false);
    let on_delete_close = move || set_delete_open(false);

    view! { cx,
        <div class="bg-slate-700 flex flex-col p-4 rounded-lg">
            <div class="flex flex-row gap-2 items-center pb-2">
                <FlexSpace />
                <h2 class="text-3xl">{ name }</h2>
                <FlexSpace />
                <Show
                    when=edit_mode
                    fallback=|_| ()
                >
                    <div class="flex flex-row gap-1 items-center">
                        <button on:click=move |_| set_form_open(true)>
                            <PencilSquareIcon />
                        </button>

                        <button on:click=move |_| set_delete_open(true)>
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
        <Show when=form_open fallback=|_| () >
            <Dialog on_close=on_form_close >
                <WidgetForm on_close=on_form_close prev_widget=widget.get_untracked() />
            </Dialog>
        </Show>
        <Show when=delete_open fallback=|_| () >
            <ConfirmationDialog
                on_confirm=move || delete_widget.dispatch(id())
                on_close=on_delete_close
            />
        </Show>
    }
}
