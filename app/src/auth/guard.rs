use leptos::*;
use leptos_router::use_navigate;

use crate::auth::use_token;

pub fn create_auth_guard() {
    let token = use_token();
    create_effect(move |_| {
        if token.get().into_inner().is_none() {
            let navigate = use_navigate();
            // workaround for navigating during initial routing
            // https://docs.rs/leptos_router/0.4.2/leptos_router/fn.use_navigate.html#panics
            request_animation_frame(move || {
                navigate("/login", Default::default());
            });
        }
    });
}
