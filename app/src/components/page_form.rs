use leptos::*;

#[component]
pub fn PageForm<F: Fn() + Copy + 'static>(cx: Scope, on_close: F) -> impl IntoView {
    view! { cx,
        <button on:click=move |_| log!("bruh") >
            "TODO"
        </button>
    }
}
