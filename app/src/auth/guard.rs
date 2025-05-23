use leptos::prelude::*;
use leptos_router::hooks::{use_location, use_navigate};

use crate::{
    auth::use_token,
    state::{use_store, Action},
};

/// Navigates to the login page if token is unset and the home page if token is
/// set to a new value.
pub fn create_auth_guard() {
    let token = use_token();
    let location = use_location();
    let store = use_store();

    Effect::new(move |_| {
        let navigate = use_navigate();
        let location = location.pathname.get();
        if token.get().into_inner().is_none() {
            // purge all user data
            store.dispatch(Action::Reset);

            // workaround for navigating during initial routing
            // https://docs.rs/leptos_router/0.4.2/leptos_router/fn.use_navigate.html#panics
            request_animation_frame(move || {
                navigate("/login", Default::default());
            });
        } else {
            // token changed to a new user, reload all data
            store.dispatch(Action::Reload);

            if location == "/login" {
                // we probably just logged in, move from login page to home
                request_animation_frame(move || {
                    navigate("/", Default::default());
                });
            }
        }
    });
}
