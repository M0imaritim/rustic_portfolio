#![recursion_limit = "512"]
pub mod components;
pub mod data;
pub mod pages;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::components::nav::Nav;
use crate::pages::{
    contact::ContactPage, home::HomePage, iot::IoTPage, not_found::NotFound, projects::ProjectsPage,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Langat Moimaritim | Software Engineer"/>
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <Meta name="og:title" content="Langat Moimaritim | Software Engineer"/>
        <Meta name="og:description" content="Full stack engineer diving deep into Rust — Leptos, Axum, and beyond."/>
        <Router>
            <Nav/>
            <main>
                <Routes fallback=|| view! { <NotFound/> }>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/projects") view=ProjectsPage/>
                    <Route path=path!("/iot") view=IoTPage/>
                    <Route path=path!("/contact") view=ContactPage/>
                </Routes>
            </main>
        </Router>
    }
}
