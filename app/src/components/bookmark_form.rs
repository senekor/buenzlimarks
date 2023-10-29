use leptos::*;
use models::{Bookmark, Id, Page, Widget};

use crate::api::{create_submit_entity, use_entities, use_filtered_entities};

#[component]
pub fn BookmarkForm<F: Fn() + Copy + 'static>(
    on_close: F,
    #[prop(optional)] prev_bookmark: Option<Bookmark>,
) -> impl IntoView {
    let is_add = prev_bookmark.is_none();
    let prev_bookmark = store_value(prev_bookmark);

    let (page_id, set_page_id) = create_signal::<Option<Id<Page>>>(None);
    let pages = use_entities::<Page>();

    if let Some(bookmark) = prev_bookmark() {
        let all_widgets = use_entities::<Widget>();
        create_effect(move |_| {
            let p_id = all_widgets()
                .unwrap_or_default()
                .into_iter()
                .find(|w| w.id == bookmark.widget_id)
                .map(|w| w.page_id);
            set_page_id(p_id);
        });
    };

    let (widget_id, set_widget_id) =
        create_signal::<Option<Id<Widget>>>(prev_bookmark().map(|b| b.widget_id));
    let widget_resource = create_memo(move |_| page_id().map(use_filtered_entities::<Widget>));
    let page_widgets =
        create_memo(move |_| widget_resource().and_then(|rsc| rsc()).unwrap_or_default());

    // So, this is a bit of a hack. When the page_widgets are updated
    // *after* widget_id has been updated, the DOM does not pick up on
    // this and doesn't display the name corresponding to widget_id
    // correctly. By forcing a pseudo-update on the widget_id signal,
    // the DOM is updated and shows the correct widget name, once
    // page_widgets are updated.
    create_effect(move |prev| {
        page_widgets.track();
        if prev.is_some() {
            set_widget_id.update(|_| {});
        }
    });

    let (name, set_name) =
        create_signal::<String>(prev_bookmark().map(|b| b.name).unwrap_or_default());
    let (url, set_url) =
        create_signal::<String>(prev_bookmark().map(|b| b.url).unwrap_or_default());

    let bookmark = Signal::derive(move || Bookmark {
        id: prev_bookmark().map(|b| b.id).unwrap_or_else(|| "".into()),
        name: name(),
        url: url(),
        widget_id: widget_id().unwrap_or_else(|| "".into()),
    });

    let submit_bookmark = create_submit_entity::<Bookmark>();

    view! {
        <select
            class="bg-slate-600 rounded p-2"
            class=("text-gray-400", move || page_id().is_none())
            prop:value=move || page_id().map(String::from).unwrap_or_default()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                if val.is_empty() {
                    batch(|| {
                        set_page_id(None);
                        if widget_id.with(|w| w.is_some()) {
                            set_widget_id(None);
                        }
                    });
                } else {
                    batch(|| {
                        set_page_id(Some(Id::from(val)));
                        if widget_id.with(|w| w.is_some()) {
                            set_widget_id(None);
                        }
                    });
                }
            }
        >
            <option value="">"Select a page"</option>
            <For
                each=move || pages().unwrap_or_default()
                key=|page| page.id.clone()
                let:page
            >
                <option value=page.id.to_string() >
                    { page.name }
                </option>
            </For>
        </select>
        <select
            class="bg-slate-600 rounded p-2"
            class=("text-gray-400", move || widget_id.with(|w| w.is_none()))
            prop:value=move || widget_id().map(String::from).unwrap_or_default()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                if val.is_empty() {
                    set_widget_id(None);
                } else {
                    set_widget_id(Some(Id::from(val)));
                }
            }
        >
            <option value="">"Select a widget"</option>
            <For
                each=page_widgets
                key=|widget| widget.id.clone()
                let:widget
            >
                <option value=widget.id.to_string() >
                    { widget.name }
                </option>
            </For>
        </select>
        <input
            class="bg-slate-600 rounded px-2 py-1.5"
            placeholder="Name"
            prop:value=name
            on:input=move |ev| set_name(event_target_value(&ev))
        />
        <input
            class="bg-slate-600 rounded px-2 py-1.5"
            placeholder="URL"
            prop:value=url
            on:input=move |ev| set_url(event_target_value(&ev))
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
                disabled=move || name.with(|n| n.is_empty()) || url.with(|u| u.is_empty())
                on:click=move |_| {
                    submit_bookmark.dispatch(bookmark.get_untracked());
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
