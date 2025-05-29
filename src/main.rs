use leptos::*;

pub mod components;
pub mod pages;
pub mod data;

mod app;
use app::App;

fn main() {
    // Better panic messages using console_error_panic_hook when available (no-op on non-wasm targets)
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! { <App/> }
    });
}