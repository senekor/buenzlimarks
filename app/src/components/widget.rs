use leptos::*;
use models::Widget as WidgetType;

use crate::{
    components::{Bookmark, ConfirmationDialog, Dialog, FlexSpace, WidgetForm},
    edit_mode::use_edit_mode,
    icons::{PencilSquareIcon, XMarkIcon},
    state::use_store,
    state::Action,
};

#[component]
pub fn Widget(widget: WidgetType) -> impl IntoView {
    let store = use_store();
    let bookmarks = store.bookmarks_by(widget.id.clone());

    let widget = store_value(widget);

    let edit_mode = use_edit_mode().read();

    let (form_open, set_form_open) = create_signal(false);
    let on_form_close = move || set_form_open(false);

    let (delete_open, set_delete_open) = create_signal(false);
    let on_delete_close = move || set_delete_open(false);

    view! {
        <div class="bg-slate-700 flex flex-col p-2 rounded-lg">
            <div class="flex flex-row gap-2 items-center pb-1">
                <FlexSpace />
                <h2 class="text-xl">{ widget().name }</h2>
                <FlexSpace />
                <Show
                    when=edit_mode
                    fallback=|| ()
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
                each=bookmarks
                key=|bookmark| bookmark.clone()
                let:bookmark
            >
                <Bookmark bookmark />
            </For>
        </div>
        <Show when=form_open fallback=|| () >
            <Dialog on_close=on_form_close >
                <WidgetForm on_close=on_form_close prev_widget=widget() />
            </Dialog>
        </Show>
        <Show when=delete_open fallback=|| () >
            <ConfirmationDialog
                on_confirm=move || store.dispatch(Action::DeleteWidget(widget()))
                on_close=on_delete_close
            />
        </Show>
    }
}
