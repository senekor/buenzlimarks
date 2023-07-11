use leptos::*;
use leptos_router::use_navigate;

use crate::{api::create_settings_resource, auth::use_token};

pub fn create_auth_guard(cx: Scope) {
    let token = use_token(cx);
    create_effect(cx, move |_| {
        if token().into_inner().is_none() {
            let navigate = use_navigate(cx);
            // workaround for navigating during initial routing
            // https://docs.rs/leptos_router/0.4.2/leptos_router/fn.use_navigate.html#panics
            request_animation_frame(move || {
                navigate("/login", Default::default()).unwrap();
            });
        }
    });
    let settings = create_settings_resource(cx);
    create_effect(cx, move |_| {
        if let Some(Err(_)) = settings.read(cx) {
            let navigate = use_navigate(cx);
            // TODO handle other errors than "unauthenticated"
            request_animation_frame(move || {
                navigate("/login", Default::default()).unwrap();
            });
        }
    });
}
