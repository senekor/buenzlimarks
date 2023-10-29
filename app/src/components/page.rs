use leptos::*;
use models::{Page as PageType, Widget as WidgetType};

use crate::{api::use_filtered_entities, components::Widget};

#[component]
pub fn Page(page: Signal<PageType>) -> impl IntoView {
    let id = Signal::derive(move || page().id);

    let widgets = use_filtered_entities::<WidgetType>(id.get_untracked());

    view! {
        <div class="flex flex-col gap-4 items-center">
            <div class="flex flex-col gap-2 items-stretch">
                <For
                    each=move || widgets().unwrap_or_default()
                    key=|widget| widget.id.clone()
                    let:widget
                >
                    <Widget widget />
                </For>
            </div>
        </div>
    }
}
