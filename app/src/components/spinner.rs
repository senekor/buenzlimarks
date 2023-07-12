use leptos::*;

#[component]
pub fn Spinner(cx: Scope) -> impl IntoView {
    view! { cx,
        // taken from
        // https://preline.co/docs/spinners.html
        <div
            class="animate-spin inline-block w-16 h-16 border-4 border-current border-t-transparent text-slate-400 rounded-full"
            role="status"
            aria-label="loading"
        />
    }
}
