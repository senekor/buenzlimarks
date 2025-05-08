use leptos::prelude::*;

use crate::{
    components::{BookmarkForm, Dialog, IconButton, PageForm, WidgetForm},
    edit_mode::use_edit_mode,
    icons::PlusIcon,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    None,
    Picking,
    Page,
    Widget,
    Bookmark,
}

impl State {
    fn is_entity(self) -> bool {
        use State::*;
        matches!(self, Page | Widget | Bookmark)
    }
}

#[component]
pub fn AddButton() -> impl IntoView {
    let edit_mode = use_edit_mode().read();

    let (state, set_state) = signal::<State>(State::None);
    Effect::new(move |prev: Option<()>| {
        if !edit_mode() && prev.is_some() {
            set_state(State::None);
        }
    });

    let on_close = move || set_state(State::None);

    view! {
        <Show
            when=edit_mode
            fallback=|| ()
        >
            <div class="absolute bottom-3 right-3">
                <IconButton on:click=move |_| set_state.update(|prev| *prev = if *prev == State::Picking {
                    State::None
                } else {
                    State::Picking
                })>
                    <PlusIcon />
                </IconButton>

                <Show
                    when=move || state() == State::Picking
                    fallback=|| ()
                >
                    <div class="absolute bottom-8 right-8
                                bg-slate-600 rounded p-4 border-2 border-white
                                flex flex-col gap-2 text-lg">
                        <button on:click=move |_| set_state(State::Page) >Page</button>
                        <button on:click=move |_| set_state(State::Widget) >Widget</button>
                        <button on:click=move |_| set_state(State::Bookmark) >Bookmark</button>
                    </div>
                </Show>

                <Show when=move || state().is_entity() fallback=|| ()>
                    <Dialog on_close>
                        <Show when=move || state() == State::Page fallback=|| ()>
                            <PageForm on_close />
                        </Show>
                        <Show when=move || state() == State::Widget fallback=|| ()>
                            <WidgetForm on_close />
                        </Show>
                        <Show when=move || state() == State::Bookmark fallback=|| ()>
                            <BookmarkForm on_close />
                        </Show>
                    </Dialog>
                </Show>
            </div>
        </Show>
    }
}
