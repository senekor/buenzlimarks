use leptos::*;
use models::Page as PageType;

use crate::{
    api::{create_delete_entity, create_submit_entity, use_entities},
    auth::{create_auth_guard, use_auth},
    components::{FlexSpace, IconButton, Page, PageTab, AddButton},
    edit_mode::use_edit_mode,
    icons::{ArrowRightOnRectangleIcon, PencilSquareIcon, QuestionMarkCircleIcon},
};

#[cfg(debug_assertions)]
static DOCS_HREF: &str = "http://localhost:5000";
#[cfg(not(debug_assertions))]
static DOCS_HREF: &str = "/docs";

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    create_auth_guard(cx);

    let auth = use_auth(cx);

    let pages = use_entities::<PageType>(cx);

    let submit_page = create_submit_entity::<PageType>(cx);
    let delete_page = create_delete_entity::<PageType>(cx);

    let (selected_page, set_selected_page) = create_signal(cx, None);
    create_effect(cx, move |_| {
        set_selected_page(pages.read(cx).and_then(|pages| pages.into_iter().next()))
    });

    let set_edit_mode = use_edit_mode(cx).write();

    view! { cx,
        <div class="h-screen flex flex-col flex-wrap gap-2">
            <div class="flex gap-2 p-2 w-full">
                <For
                    each=move || pages.read(cx).unwrap_or_default()
                    key=|page| page.id.clone()
                    view=move |cx, page| {
                        let id = store_value(cx, page.id.clone());
                        let is_selected = Signal::derive(cx, move || {
                            selected_page().is_some_and(|sp| sp.id == id())
                        });
                        let select = SignalSetter::map(cx, move |p| set_selected_page(Some(p)));
                        view! { cx, <PageTab
                            page
                            is_selected
                            select
                            submit_page
                            delete_page
                        /> }
                    }
                />
                <FlexSpace />
                // <IconButton on:click=move |_| {
                //     submit_page.dispatch(PageType {
                //         id: "".into(),
                //         name: "new page".into(),
                //     })
                // }>
                //     <PlusIcon />
                // </IconButton>
                <IconButton on:click=move |_| set_edit_mode.update(|prev| *prev = !*prev) >
                    <PencilSquareIcon />
                </IconButton>
                <IconButton on:click=move |_| auth.logout() >
                    <ArrowRightOnRectangleIcon />
                </IconButton>
                <a
                    class="bg-slate-600 rounded-full p-2 w-min"
                    href=DOCS_HREF
                    rel="external" // make sure leptos doesn't use client-side routing
                >
                    <QuestionMarkCircleIcon />
                </a>
            </div>
            <Suspense fallback=move || "">{ move || {
                let sp = selected_page()?;
                let page = Signal::derive(cx, move || sp.clone());
                Some(view! { cx, <Page page /> })
            }}</Suspense>
            <AddButton />
        </div>
    }
}
