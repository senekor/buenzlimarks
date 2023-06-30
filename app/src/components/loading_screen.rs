use leptos::*;

use crate::components::{FlexSpace, Spinner};

#[component]
pub fn LoadingScreen(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="flex flex-col h-screen text-white items-center justify-center">
            <FlexSpace />
            <Spinner />
            <FlexSpace />
            <FlexSpace />
        </div>
    }
}
