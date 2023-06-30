use leptos::*;
use models::Page as PageType;

use crate::{
    api::{create_delete_entity, create_submit_entity, use_entities},
    auth::use_logout,
    components::{FlexSpace, IconButton, LoadingScreen, Page},
    icons::{ArrowRightOnRectangleIcon, PlusIcon, XMarkIcon},
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let logout = use_logout(cx);
    let logout = Signal::derive(cx, move || logout.clone());

    let pages = use_entities::<PageType>(cx);

    let submit_page = create_submit_entity::<PageType>(cx);
    let delete_page = create_delete_entity::<PageType>(cx);

    let (selected_page, set_selected_page) = create_signal(cx, None);
    create_effect(cx, move |_| {
        set_selected_page(pages.read(cx).and_then(|pages| pages.into_iter().next()))
    });

    view! { cx,
        <Suspense
            fallback=move || LoadingScreen(cx)
        >
            <div class="h-screen flex flex-col flex-wrap">
                <div class="flex gap-2 p-2 w-full">
                    <For
                        each=move || pages.read(cx).unwrap_or_default()
                        key=|page| page.id.clone()
                        view=move |cx, page| {
                            let page = Signal::derive(cx, move || page.clone());
                            let id = Signal::derive(cx, move || page().id);
                            let is_selected = move || selected_page().is_some_and(|sp| sp.id == id());
                            let not_selected = move || !is_selected();
                            view! { cx,
                                <button
                                    class="rounded-lg pl-3 pr-2 py-1 flex flex-row place-items-center gap-2"
                                    class=("bg-orange-800", is_selected)
                                    class=("bg-slate-600", not_selected)
                                    on:click=move |_| set_selected_page(Some(page()))
                                >
                                    { move || page().name }
                                    // TODO edit functionality like for widget name
                                    <XMarkIcon on:click=move |ev| {
                                        delete_page.dispatch(id.get_untracked());
                                        ev.stop_propagation();
                                    } />
                                </button>
                            }
                        }
                    />
                    <FlexSpace />
                    <IconButton
                        on:click=move |_| {
                            submit_page.dispatch(PageType {
                                id: "".into(),
                                name: "new page".into(),
                            })
                        }
                    >
                        <PlusIcon />
                    </IconButton>
                    <IconButton on:click=move |_| logout().logout() >
                        <ArrowRightOnRectangleIcon />
                    </IconButton>
                </div>
                <Suspense fallback=move || "">{ move || {
                    let sp = selected_page()?;
                    let page = Signal::derive(cx, move || sp.clone());
                    Some(view! { cx, <Page page /> })
                }}</Suspense>
            </div>
        </Suspense>
    }
}
