use leptos::*;
use models::Page as PageType;

use crate::{
    components::{ConfirmationDialog, Dialog, PageForm},
    edit_mode::use_edit_mode,
    icons::{PencilSquareIcon, XMarkIcon},
    state::use_store,
    state::Action,
};

#[component]
pub fn PageTab(
    page: PageType,
    is_selected: Signal<bool>,
    select: SignalSetter<PageType>,
) -> impl IntoView {
    let store = use_store();

    let page = store_value(page);

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
            { page().name }
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

        <Show when=form_open fallback=|| () >
            <Dialog on_close >
                <PageForm on_close prev_page=page() />
            </Dialog>
        </Show>

        <Show when=delete_open fallback=|| () >
            <ConfirmationDialog
                on_confirm=move || store.dispatch(Action::DeletePage(page()))
                on_close=on_delete_close
            />
        </Show>
    }
}
