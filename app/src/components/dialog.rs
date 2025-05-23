use leptos::prelude::*;

#[component]
pub fn Dialog<F>(children: Children, on_close: F) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    view! {
        <div class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2
                    bg-slate-700 rounded p-8 border-2 border-white
                    flex flex-col gap-2
                    shadow-2x shadow-slate-900
                    z-20"
        >
            { children() }
        </div>
        <div
            class="fixed top-0 left-0 h-screen w-screen backdrop-brightness-75"
            on:click=move |_| on_close()
        />
    }
}
