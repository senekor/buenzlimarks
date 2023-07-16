use leptos::*;
use models::Page as PageType;

use crate::{
    api::{use_entity, Delete, Submit},
    icons::{PencilSquareIcon, XMarkIcon}, edit_mode::use_edit_mode,
};

#[component]
pub fn PageTab(
    cx: Scope,
    page: PageType,
    is_selected: Signal<bool>,
    select: SignalSetter<PageType>,
    submit_page: Submit<PageType>,
    delete_page: Delete<PageType>,
) -> impl IntoView {
    let id = store_value(cx, page.id.clone());
    let page = use_entity(cx, page);

    let name = move || page().name;
    let not_selected = move || !is_selected();

    let (name_form, set_name_form) = create_signal::<Option<String>>(cx, None);

    let edit_mode = use_edit_mode(cx).read();
    let no_edit_mode = Signal::derive(cx, move || !edit_mode());

    view! { cx,
        <button
            class="rounded-lg px-3 flex flex-row place-items-center gap-1"
            class=("bg-orange-800", is_selected)
            class=("bg-slate-600", not_selected)
            on:click=move |_| select(page())
        >
            <p hidden=move || name_form().is_some() >{ name }</p>
            <input
                class="bg-inherit p-1 px-2 rounded"
                class=("bg-orange-700", is_selected)
                class=("bg-slate-500", not_selected)
                hidden=move || name_form().is_none()
                prop:value=name_form
                on:input=move |ev| { set_name_form(Some(event_target_value(&ev))); }
                on:keydown=move |ev| {
                    if &ev.key() == "Enter" {
                        submit_page.dispatch(PageType {
                            id: id(),
                            name: name_form.get_untracked().unwrap_or_default(),
                        });
                        set_name_form(None);
                    }
                }
                on:click=move |ev| ev.stop_propagation()
            />
            <button class="pl-2" hidden=no_edit_mode on:click=move |ev| {
                set_name_form(Some(page.get_untracked().name));
                ev.stop_propagation();
            }>
                <PencilSquareIcon />
            </button>
            <button hidden=no_edit_mode on:click=move |ev| {
                delete_page.dispatch(id());
                ev.stop_propagation();
            }>
                <XMarkIcon />
            </button>
        </button>
    }
}
