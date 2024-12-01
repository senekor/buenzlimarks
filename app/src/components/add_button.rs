use leptos::*;

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

    let (state, set_state) = create_signal::<State>(State::None);
    create_effect(move |prev| {
        if !edit_mode.get() && prev.is_some() {
            set_state.set(State::None);
        }
    });

    let on_close = move || set_state.set(State::None);

    view! {
        <Show
            when=move || edit_mode.get()
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
                    when=move || state.get() == State::Picking
                    fallback=|| ()
                >
                    <div class="absolute bottom-8 right-8
                                bg-slate-600 rounded p-4 border-2 border-white
                                flex flex-col gap-2 text-lg">
                        <button on:click=move |_| set_state.set(State::Page) >Page</button>
                        <button on:click=move |_| set_state.set(State::Widget) >Widget</button>
                        <button on:click=move |_| set_state.set(State::Bookmark) >Bookmark</button>
                    </div>
                </Show>

                <Show when=move || state.get().is_entity() fallback=|| ()>
                    <Dialog on_close>
                        <Show when=move || state.get() == State::Page fallback=|| ()>
                            <PageForm on_close />
                        </Show>
                        <Show when=move || state.get() == State::Widget fallback=|| ()>
                            <WidgetForm on_close />
                        </Show>
                        <Show when=move || state.get() == State::Bookmark fallback=|| ()>
                            <BookmarkForm on_close />
                        </Show>
                    </Dialog>
                </Show>
            </div>
        </Show>
    }
}
