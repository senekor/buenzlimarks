use leptos::*;
use models::{Id, Page, Widget};

use crate::state::{use_store, Action};

#[component]
pub fn WidgetForm<F: Fn() + Copy + 'static>(
    on_close: F,
    #[prop(optional)] prev_widget: Option<Widget>,
) -> impl IntoView {
    let store = use_store();

    let is_add = prev_widget.is_none();
    let prev_widget = store_value(prev_widget);

    let (page_id, set_page_id) =
        create_signal::<Option<Id<Page>>>(prev_widget.get_value().map(|w| w.page_id));
    let pages = store.pages();

    // Force DOM update when pages are fetched such that page name is
    // displayed correctly.
    create_effect(move |prev| {
        pages.with(|_| ()); // track page updates
        if prev.is_some() {
            request_animation_frame(move || {
                set_page_id.update(|_| {});
            });
        }
    });

    let (name, set_name) =
        create_signal::<String>(prev_widget.get_value().map(|b| b.name).unwrap_or_default());

    let widget = Signal::derive(move || Widget {
        id: prev_widget
            .get_value()
            .map(|b| b.id)
            .unwrap_or_else(|| "".into()),
        name: name.get(),
        page_id: page_id.get().unwrap_or_else(|| "".into()),
    });

    view! {
        <select
            class="bg-slate-600 rounded p-2"
            class=("text-gray-400", move || page_id.get().is_none())
            prop:value=move || page_id.get().map(String::from).unwrap_or_default()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                if val.is_empty() {
                    set_page_id.set(None);
                } else {
                    set_page_id.set(Some(Id::from(val)));
                }
            }
        >
            <option value="">"Select a page"</option>
            <For
                each=move || pages.get()
                key=|page| page.id.clone()
                let:page
            >
                <option value=page.id.to_string() >
                    { page.name }
                </option>
            </For>
        </select>
        <input
            class="bg-slate-600 rounded px-2 py-1.5"
            placeholder="Name"
            prop:value=name
            on:input=move |ev| set_name.set(event_target_value(&ev))
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
                disabled=move || name.with(|n| n.is_empty()) || page_id.with(|n| n.is_none())
                on:click=move |_| {
                    store.dispatch(Action::SubmitWidget(widget.get_untracked()));
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
