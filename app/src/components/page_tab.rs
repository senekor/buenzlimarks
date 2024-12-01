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

    let not_selected = move || !is_selected.get();

    let edit_mode = use_edit_mode().read();
    let no_edit_mode = Signal::derive(move || !edit_mode.get());

    let (form_open, set_form_open) = create_signal(false);
    let on_close = move || set_form_open.set(false);

    let (delete_open, set_delete_open) = create_signal(false);
    let on_delete_close = move || set_delete_open.set(false);

    view! {
        <button
            class="rounded-lg px-3 flex flex-row place-items-center gap-1"
            class=("bg-orange-800", is_selected)
            class=("bg-slate-600", not_selected)
            on:click=move |_| select.set(page.get_value())
        >
            { page.get_value().name }
            <button class="pl-2" hidden=no_edit_mode on:click=move |ev| {
                set_form_open.set(true);
                ev.stop_propagation();
            }>
                <PencilSquareIcon />
            </button>

            <button hidden=no_edit_mode on:click=move |ev| {
                set_delete_open.set(true);
                ev.stop_propagation();
            }>
                <XMarkIcon />
            </button>
        </button>

        <Show when=move || form_open.get() fallback=|| () >
            <Dialog on_close >
                <PageForm on_close prev_page=page.get_value() />
            </Dialog>
        </Show>

        <Show when=move || delete_open.get() fallback=|| () >
            <ConfirmationDialog
                on_confirm=move || store.dispatch(Action::DeletePage(page.get_value()))
                on_close=on_delete_close
            />
        </Show>
    }
}
