use leptos::*;
use leptos_router::use_query_map;

use crate::{auth::use_auth, components::LoadingScreen};

#[component]
pub fn GithubCallback(cx: Scope) -> impl IntoView {
    let query_params = use_query_map(cx);
    let auth = use_auth(cx);

    // TODO this login request is sent twice. The second time fails
    // obviously, since the auth flow artifacts are only valid once.
    // I have no idea why this components renders twice.
    //
    // TODO this was noted for the react app, is this still true
    // for leptos?
    auth.login(format!(
        "/api/auth/github/callback{}",
        query_params.get_untracked().to_query_string(),
    ));

    view! { cx, <LoadingScreen/> }
}
