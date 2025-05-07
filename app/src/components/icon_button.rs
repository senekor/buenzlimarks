use leptos::prelude::*;

#[component]
pub fn IconButton(children: Children) -> impl IntoView {
    view! {
        <button class="bg-slate-600 rounded-full p-2 w-min">
            { children() }
        </button>
    }
}
