use leptos::*;
use models::Page;

use crate::state::{use_store, Action};

#[component]
pub fn PageForm<F: Fn() + Copy + 'static>(
    on_close: F,
    #[prop(optional)] prev_page: Option<Page>,
) -> impl IntoView {
    let store = use_store();

    let is_add = prev_page.is_none();
    let prev_page = store_value(prev_page);

    let (name, set_name) = create_signal::<String>(prev_page().map(|b| b.name).unwrap_or_default());

    let page = Signal::derive(move || Page {
        id: prev_page().map(|b| b.id).unwrap_or_else(|| "".into()),
        name: name(),
    });

    view! {
        <input
            class="bg-slate-600 rounded px-2 py-1.5"
            placeholder="Name"
            prop:value=name
            on:input=move |ev| set_name(event_target_value(&ev))
        />
        <div class="self-center flex gap-2">
            <button
                class="bg-slate-600 w-fit rounded px-1"
                on:click=move |_| on_close()
            >
                Cancel
            </button>
            <button
                class="bg-slate-600 w-fit rounded px-1 disabled:text-gray-400"
                disabled=move || name.with(|n| n.is_empty())
                on:click=move |_| {
                    store.dispatch(Action::SubmitPage(page.get_untracked()));
                    on_close();
                }
            >{
                move || if is_add {
                    "Add"
                } else {
                    "Save"
                }
            }</button>
        </div>
    }
}
