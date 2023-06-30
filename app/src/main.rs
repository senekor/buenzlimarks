use leptos::mount_to_body;

mod api;
mod auth;
mod components;
mod icons;

use api::provide_api_context;
use auth::provide_auth_context;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(|cx| {
        provide_auth_context(cx);
        provide_api_context(cx);
        components::App(cx)
    })
}
