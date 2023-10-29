use leptos::*;
use leptos_router::*;

use crate::{
    api::provide_api_context,
    auth::{provide_auth_context, GithubCallback, Login},
    components::Home,
    edit_mode::provide_edit_mode,
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
    provide_api_context();
    provide_edit_mode();

    children()
}

#[component]
pub fn AppRoutes() -> impl IntoView {
    view! {
        <Routes>
            <Route path="/" view=|| view! { <Home/> }/>
            <Route path="/login" view=|| view! { <Login/> }/>
            <Route path="/auth/github/callback" view=|| view! { <GithubCallback/> }/>
        </Routes>
    }
}
