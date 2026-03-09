pub mod app;
pub mod components;
pub mod data;
pub mod pages;

#[cfg(feature = "hydrate")]
mod hydrate {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn main() {
        use crate::app::App;
        leptos::mount::mount_to_body(App);
    }
}