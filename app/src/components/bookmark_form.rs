use leptos::*;
use models::{Bookmark, Id, Page, Widget};

use crate::api::{create_submit_entity, use_entities, use_filtered_entities};

#[component]
pub fn BookmarkForm<F: Fn() + Copy + 'static>(
    cx: Scope,
    on_close: F,
    #[prop(optional)] prev_bookmark: Option<Bookmark>,
) -> impl IntoView {
    let is_add = prev_bookmark.is_none();
    let prev_bookmark = store_value(cx, prev_bookmark);

    let (page_id, set_page_id) = create_signal::<Option<Id<Page>>>(cx, None);
    let pages = use_entities::<Page>(cx);

    if let Some(bookmark) = prev_bookmark() {
        let all_widgets = use_entities::<Widget>(cx);
        create_effect(cx, move |_| {
            let p_id = all_widgets
                .read(cx)
                .unwrap_or_default()
                .into_iter()
                .find(|w| w.id == bookmark.widget_id)
                .map(|w| w.page_id);
            set_page_id(p_id);
        })
    };

    let (widget_id, set_widget_id) =
        create_signal::<Option<Id<Widget>>>(cx, prev_bookmark().map(|b| b.widget_id));
    let widget_resource = create_memo(cx, move |_| {
        page_id().map(|p_id| use_filtered_entities::<Widget>(cx, p_id))
    });
    let page_widgets = create_memo(cx, move |_| {
        widget_resource()
            .and_then(|rsc| rsc.read(cx))
            .unwrap_or_default()
    });

    // So, this is a bit of a hack. When the page_widgets are updated
    // *after* widget_id has been updated, the DOM does not pick up on
    // this and doesn't display the name corresponding to widget_id
    // correctly. By forcing a pseudo-update on the widget_id signal,
    // the DOM is updated and shows the correct widget name, once
    // page_widgets are updated.
    create_effect(cx, move |prev| {
        page_widgets.track();
        if prev.is_some() {
            set_widget_id.update(|_| {});
        }
    });

    let (name, set_name) =
        create_signal::<String>(cx, prev_bookmark().map(|b| b.name).unwrap_or_default());
    let (url, set_url) =
        create_signal::<String>(cx, prev_bookmark().map(|b| b.url).unwrap_or_default());

    let bookmark = Signal::derive(cx, move || Bookmark {
        id: prev_bookmark().map(|b| b.id).unwrap_or_else(|| "".into()),
        name: name(),
        url: url(),
        widget_id: widget_id().unwrap_or_else(|| "".into()),
    });

    let submit_bookmark = create_submit_entity::<Bookmark>(cx);

    view! { cx,
        <select
            class="bg-slate-600 rounded p-2"
            class=("text-gray-400", move || page_id().is_none())
            prop:value=move || page_id().map(String::from).unwrap_or_default()
            on:input=move |ev| {
                let val = event_target_value(&ev);
                if val.is_empty() {
                    cx.batch(|| {
                        set_page_id(None);
                        if widget_id.with(|w| w.is_some()) {
                            set_widget_id(None);
                        }
                    });
                } else {
                    cx.batch(|| {
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
                view=move |cx, widget| {
                    view! { cx,
                        <option value=widget.id.to_string() >
                            { widget.name }
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