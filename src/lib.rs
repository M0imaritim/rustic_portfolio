pub mod app;
pub mod components;
pub mod data;
pub mod pages;

#[cfg(feature = "hydrate")]
mod hydrate {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn main() {
        use crate::app::App;
        _ = console_log::init_with_level(log::Level::Debug); // optional
        leptos::mount::mount_to_body(App);
    }
}