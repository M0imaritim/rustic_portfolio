use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

use crate::pages::{home::HomePage, projects::ProjectsPage, iot::IoTPage, contact::ContactPage, not_found::NotFound};
use crate::components::nav::Nav;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Langat Moimaritim | Software Engineer"/>
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