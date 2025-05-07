use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::{
    auth::{provide_auth_context, GithubCallback, Login},
    components::Home,
    edit_mode::provide_edit_mode,
    state::provide_store,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Providers>
                <AppRoutes/>
            </Providers>
        </Router>
    }
}

#[component]
pub fn Providers(children: Children) -> impl IntoView {
    provide_auth_context();
    provide_edit_mode();
    provide_store();

    children()
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <FlatRoutes fallback=|| "Not found.">
            <Route path=path!("/") view=|| view! { <Home/> }/>
            <Route path=path!("/login") view=|| view! { <Login/> }/>
            <Route path=path!("/auth/github/callback") view=|| view! { <GithubCallback/> }/>
        </FlatRoutes>
    }
}
