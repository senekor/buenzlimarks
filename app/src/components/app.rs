use leptos::*;
use leptos_router::*;

use crate::{
    api::provide_api_context,
    auth::{provide_auth_context, GithubCallback, Login},
    components::Home,
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Providers>
                <AppRoutes/>
            </Providers>
        </Router>
    }
}

#[component]
pub fn Providers(cx: Scope, children: Children) -> impl IntoView {
    provide_auth_context(cx);
    provide_api_context(cx);

    children(cx)
}

#[component]
pub fn AppRoutes(cx: Scope) -> impl IntoView {
    view! { cx,
        <Routes>
            <Route path="/" view=|cx| view! { cx, <Home/> }/>
            <Route path="/login" view=|cx| view! { cx, <Login/> }/>
            <Route path="/auth/github/callback" view=|cx| view! { cx, <GithubCallback/> }/>
        </Routes>
    }
}
