use leptos::*;
use leptos_router::use_navigate;

use crate::{api::create_settings_resource, auth::use_token};

pub fn create_auth_guard() {
    let token = use_token();
    create_effect(move |_| {
        if token().into_inner().is_none() {
            let navigate = use_navigate();
            // workaround for navigating during initial routing
            // https://docs.rs/leptos_router/0.4.2/leptos_router/fn.use_navigate.html#panics
            request_animation_frame(move || {
                navigate("/login", Default::default());
            });
        }
    });
    let settings = create_settings_resource();
    create_effect(move |_| {
        if let Some(Err(_)) = settings() {
            let navigate = use_navigate();
            // TODO handle other errors than "unauthenticated"
            request_animation_frame(move || {
                navigate("/login", Default::default());
            });
        }
    });
}
