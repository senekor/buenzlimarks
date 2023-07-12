use leptos::*;
use models::{Page as PageType, Widget as WidgetType};

use crate::{
    api::{create_submit_entity, use_filtered_entities},
    components::{IconButton, Widget},
    icons::PlusIcon,
};

#[component]
pub fn Page(cx: Scope, page: Signal<PageType>) -> impl IntoView {
    let id = Signal::derive(cx, move || page().id);

    let widgets = use_filtered_entities::<WidgetType>(cx, id.get_untracked());

    let submit_widget = create_submit_entity::<WidgetType>(cx);

    view! { cx,
        <div class="flex flex-col gap-4 items-center">
            <div class="flex flex-col gap-2 items-stretch">
                <For
                    each=move || widgets.read(cx).unwrap_or_default()
                    key=|widget| widget.id.clone()
                    view=move |cx, widget| {
                        view! { cx, <Widget widget /> }
                    }
                />
            </div>
            <IconButton
                on:click=move |_| {
                    submit_widget.dispatch(WidgetType {
                        id: "".into(),
                        name: "new widget".into(),
                        page_id: id.get_untracked(),
                    })
                }
            >
                <PlusIcon />
            </IconButton>
        </div>
    }
}
