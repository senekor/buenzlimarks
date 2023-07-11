use leptos::*;
use leptos_router::use_navigate;

use crate::{api::create_settings_resource, auth::use_token};

pub fn create_auth_guard(cx: Scope) {
    let navigate = use_navigate(cx);
    let token = use_token(cx);
    create_effect(cx, move |_| {
        if token().into_inner().is_none() {
            navigate("/login", Default::default()).unwrap();
        }
    });
    let navigate = use_navigate(cx);
    let settings = create_settings_resource(cx);
    create_effect(cx, move |_| {
        if let Some(Err(_)) = settings.read(cx) {
            // TODO handle other errors than "unauthenticated"
            navigate("/login", Default::default()).unwrap();
        }
    });
}
