use leptos::*;
use models::{Id, Page, Widget};

use crate::api::{create_submit_entity, use_entities};

#[component]
pub fn WidgetForm<F: Fn() + Copy + 'static>(
    cx: Scope,
    on_close: F,
    #[prop(optional)] prev_widget: Option<Widget>,
) -> impl IntoView {
    let is_add = prev_widget.is_none();
    let prev_widget = store_value(cx, prev_widget);

    let (page_id, set_page_id) =
        create_signal::<Option<Id<Page>>>(cx, prev_widget().map(|w| w.page_id));
    let pages = use_entities::<Page>(cx);

    // Force DOM update when pages are fetched such that page name is
    // displayed correctly.
    create_effect(cx, move |prev| {
        pages.with(cx, |_| ()); // track page updates
        if prev.is_some() {
            request_animation_frame(move || {
                set_page_id.update(|_| {});
            });
        }
    });

    let (name, set_name) =
        create_signal::<String>(cx, prev_widget().map(|b| b.name).unwrap_or_default());

    let widget = Signal::derive(cx, move || Widget {
        id: prev_widget().map(|b| b.id).unwrap_or_else(|| "".into()),
        name: name(),
        page_id: page_id().unwrap_or_else(|| "".into()),
    });

    let submit_widget = create_submit_entity::<Widget>(cx);

    view! { cx,
        <select
            class="bg-slate-600 rounded p-2"
            class=("text-gray-400", move || page_id().is_none())
            prop:value=move || page_id().map(String::from).unwrap_or_default()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                if val.is_empty() {
                    set_page_id(None);
                } else {
                    set_page_id(Some(Id::from(val)));
                }
            }
        >
            <option value="">"Select a page"</option>
            <For
                each=move || pages.read(cx).unwrap_or_default()
                key=|page| page.id.clone()
                view=move |cx, page| {
                    view! { cx,
                        <option value=page.id.to_string() >
                            { page.name }
                        </option>
                    }
                }
            />
        </select>
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
                    submit_widget.dispatch(widget.get_untracked());
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
