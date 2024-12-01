use leptos::*;
use models::Page as PageType;

use crate::{components::Widget, state::use_store};

#[component]
pub fn Page(page: Signal<PageType>) -> impl IntoView {
    let store = use_store();
    let widgets = Signal::derive(move || {
        let page_id = page.get().id;
        store.widgets_by(page_id).get()
    });

    view! {
        <div class="flex flex-col items-center">
            <div class="flex flex-col gap-2">
                <For
                    each=move || widgets.get()
                    key=|widget| widget.clone()
                    let:widget
                >
                    <Widget widget />
                </For>
            </div>
        </div>
    }
}
