use leptos::*;
use models::Page as PageType;

use crate::{
    auth::{create_auth_guard, use_auth},
    components::{AddButton, FlexSpace, IconButton, Page, PageTab},
    edit_mode::use_edit_mode,
    icons::{ArrowRightOnRectangleIcon, PencilSquareIcon, QuestionMarkCircleIcon},
    state::use_store,
};

#[cfg(debug_assertions)]
static DOCS_HREF: &str = "http://localhost:5000";
#[cfg(not(debug_assertions))]
static DOCS_HREF: &str = "/docs";

#[component]
pub fn Home() -> impl IntoView {
    create_auth_guard();

    let auth = use_auth();

    let pages = use_store().pages();

    let (selected_page, set_selected_page) = create_signal::<Option<PageType>>(None);
    create_effect(move |_| {
        pages.with(move |pages| {
            if let Some(sel) = selected_page.get_untracked() {
                if !pages.iter().any(|p| p.id == sel.id) {
                    // selected page doesn't exist anymore, was probably deleted.
                    // set it to the first page (or none).
                    set_selected_page(pages.iter().next().cloned());
                };
            } else if let Some(first_page) = pages.iter().next() {
                // no page was selected, but a page exists. select it.
                set_selected_page(Some(first_page.clone()));
            }
        });
    });

    let set_edit_mode = use_edit_mode().write();

    view! {
        <div class="h-screen flex flex-col flex-wrap gap-2">

            <div class="flex gap-2 p-2 w-full">
                <For
                    each=pages
                    key=|page| page.clone()
                    let:page
                >
                {
                    let id = store_value( page.id.clone());
                    let is_selected = Signal::derive( move || {
                        selected_page().is_some_and(|sp| sp.id == id())
                    });
                    let select = SignalSetter::map( move |p| set_selected_page(Some(p)));
                    view! {
                        <PageTab
                            page
                            is_selected
                            select
                        />
                    }
                }
                </For>

                <FlexSpace />

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

            <Suspense fallback=move || "">
                { move || {
                    let sp = selected_page()?;
                    let page = Signal::derive( move || sp.clone());
                    Some(view! { <Page page /> })
                }}
            </Suspense>

            <AddButton />

        </div>
    }
}
