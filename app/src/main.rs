use leptos::mount_to_body;

mod api;
mod auth;
mod components;
mod icons;
mod edit_mode;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    mount_to_body(components::App)
}
