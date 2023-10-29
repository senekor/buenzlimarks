use leptos::*;
use models::Page as PageType;

use crate::{
    api::{use_entity, Delete},
    components::{ConfirmationDialog, Dialog, PageForm},
    edit_mode::use_edit_mode,
    icons::{PencilSquareIcon, XMarkIcon},
};

#[component]
pub fn PageTab(
    page: PageType,
    is_selected: Signal<bool>,
    select: SignalSetter<PageType>,
    delete_page: Delete<PageType>,
) -> impl IntoView {
    let id = store_value(page.id.clone());
    let page = use_entity(page);

    let name = move || page().name;
    let not_selected = move || !is_selected();

    let edit_mode = use_edit_mode().read();
    let no_edit_mode = Signal::derive(move || !edit_mode());

    let (form_open, set_form_open) = create_signal(false);
    let on_close = move || set_form_open(false);

    let (delete_open, set_delete_open) = create_signal(false);
    let on_delete_close = move || set_delete_open(false);

    view! {
        <button
            class="rounded-lg px-3 flex flex-row place-items-center gap-1"
            class=("bg-orange-800", is_selected)
            class=("bg-slate-600", not_selected)
            on:click=move |_| select(page())
        >
            { name }
            <button class="pl-2" hidden=no_edit_mode on:click=move |ev| {
                set_form_open(true);
                ev.stop_propagation();
            }>
                <PencilSquareIcon />
            </button>

            <button hidden=no_edit_mode on:click=move |ev| {
                set_delete_open(true);
                ev.stop_propagation();
            }>
                <XMarkIcon />
            </button>
        </button>

        <Show when=form_open fallback=|_| () >
            <Dialog on_close >
                <PageForm on_close prev_page=page.get_untracked() />
            </Dialog>
        </Show>

        <Show when=delete_open fallback=|_| () >
            <ConfirmationDialog
                on_confirm=move || delete_page.dispatch(id())
                on_close=on_delete_close
            />
        </Show>
    }
}
