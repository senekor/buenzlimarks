use leptos::*;
use models::{Bookmark as BookmarkType, Id, Widget as WidgetType};

use crate::{
    api::{create_delete_entity, create_submit_entity, use_entity, use_filtered_entities},
    components::{Bookmark, FlexSpace},
    icons::{PencilSquareIcon, XMarkIcon},
};

fn bookmark_tmpl(widget_id: Id<WidgetType>) -> BookmarkType {
    BookmarkType {
        id: Id::random(),
        name: "".into(),
        url: "".into(),
        widget_id,
    }
}

#[component]
pub fn Widget(cx: Scope, widget: WidgetType) -> impl IntoView {
    let original_widget = store_value(cx, widget);
    let id = Signal::derive(cx, move || original_widget().id);

    let widget_resource = use_entity::<WidgetType>(cx, id.get_untracked());
    let widget = create_memo(cx, move |_| match widget_resource.read(cx).flatten() {
        Some(widget) => widget,
        None => original_widget(),
    });

    let name = Signal::derive(cx, move || widget().name);
    let page_id = Signal::derive(cx, move || widget().page_id);

    let bookmarks = use_filtered_entities::<BookmarkType>(cx, id.get_untracked());

    let submit_widget = create_submit_entity::<WidgetType>(cx);
    let delete_widget = create_delete_entity::<WidgetType>(cx);
    let submit_bookmark = create_submit_entity::<BookmarkType>(cx);
    let delete_bookmark = create_delete_entity::<BookmarkType>(cx);

    // const [nameForm, setNameForm] = useState<string>();
    let (name_form, set_name_form) = create_signal::<Option<String>>(cx, None);

    // const [bookmarkForm, setBookmarkForm] = useState(bookmarkTmpl(id));
    let (bookmark_form, set_bookmark_form) = create_signal(cx, bookmark_tmpl(id.get_untracked()));
    // const resetForm = useCallback(() => setBookmarkForm(bookmarkTmpl(id)), [id]);
    let reset_bookmark_form = move || set_bookmark_form(bookmark_tmpl(id()));

    view! { cx,
        <div class="bg-slate-700 flex flex-col p-4 rounded-lg">
            <div class="flex flex-row gap-2 items-center pb-2">
                <h2 class="text-3xl" hidden=move || name_form().is_some() >{ name }</h2>
                <input
                    class="bg-slate-600 p-1 px-2 rounded text-lg"
                    hidden=move || name_form().is_none()
                    prop:value=name_form
                    focus
                    on:input=move |ev| { set_name_form(Some(event_target_value(&ev))); }
                    on:keydown=move |ev| {
                        if &ev.key() == "Enter" {
                            submit_widget.dispatch(WidgetType {
                                id: id.get_untracked(),
                                name: name_form.get_untracked().unwrap_or_default(),
                                page_id: page_id.get_untracked(),
                            });
                            set_name_form(None);
                        }
                    }
                />
                <FlexSpace />
                <div class="flex flex-row gap-1 items-center">
                    <button on:click=move |_| set_name_form(Some(name()))>
                        <PencilSquareIcon />
                    </button>
                    <button on:click=move |_| delete_widget.dispatch(id())>
                        <XMarkIcon />
                    </button>
                </div>
            </div>
            <For
                each=move || bookmarks.read(cx).unwrap_or_default()
                key=|bookmark| bookmark.id.clone()
                view=move |cx, bookmark| {
                    view! { cx, <Bookmark bookmark /> }
                }
            />
            // {bookmarks?.map((b) => (
            //     <div key={b.id} class="flex w-full gap-1">
            //     <FlexSpace />
            //     <Bookmark bookmark={b} />
            //     <FlexSpace />
            //     <PencilSquareIcon
            //         class="w-6 ml-2"
            //         onClick={() => setBookmarkForm(b)}
            //     />
            //     <XMarkIcon class="w-6" onClick={() => deleteBookmark(b.id)} />
            //     </div>
            // ))}
            <input
                class="self-center w-full bg-slate-600 p-1 rounded text-white mb-1 mt-2"
                placeholder="Name"
                // value={bookmarkForm.name}
                // onInput={(e) =>
                // setBookmarkForm({ ...bookmarkForm, name: e.currentTarget.value })
                // }
            />
            <input
                class="self-center w-full bg-slate-600 p-1 rounded text-white mb-2"
                placeholder="URL"
                // value={bookmarkForm.url}
                // onInput={(e) =>
                // setBookmarkForm({ ...bookmarkForm, url: e.currentTarget.value })
                // }
            />
            <div class="self-center flex gap-2">
                // {bookmarkForm.id && (
                // <button
                //     class="text-white bg-slate-600 w-fit rounded px-1"
                //     onClick={resetForm}
                // >
                //     Cancel
                // </button>
                // )}
                // <button
                // class="text-white bg-slate-600 w-fit rounded px-1 disabled:text-gray-400"
                // disabled={!(bookmarkForm.name && bookmarkForm.url)}
                // onClick={() => submitBookmark(bookmarkForm, { onSuccess: resetForm })}
                // >
                // {!bookmarkForm.id ? "Add" : "Save"}
                // </button>
            </div>
        </div>
    }
}
