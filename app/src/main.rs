mod auth;
mod components;
mod edit_mode;
mod icons;
mod state;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    leptos::mount::mount_to_body(components::App)
}
