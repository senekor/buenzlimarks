use leptos::*;

#[component]
pub fn IconButton(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <button class="bg-slate-600 rounded-full p-2 w-min">
            { children(cx) }
        </button>
    }
}
