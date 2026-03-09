use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Nav() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    view! {
        <nav class="nav">
            <div class="nav-inner">
                <A href="/" attr:class="nav-logo">
                    <span class="nav-logo-bracket">"["</span>
                    <span class="nav-logo-name">"LM"</span>
                    <span class="nav-logo-bracket">"]"</span>
                    <span class="nav-logo-cursor">"_"</span>
                </A>

                <div class="nav-links">
                    <A href="/" attr:class="nav-link">"~/about"</A>
                    <A href="/projects" attr:class="nav-link">"~/projects"</A>
                    <A href="/iot" attr:class="nav-link">"~/iot"</A>
                    <A href="/contact" attr:class="nav-link">"~/contact"</A>
                </div>

                <button
                    class="nav-hamburger"
                    on:click=move |_| set_menu_open.update(|v| *v = !*v)
                    aria-label="Toggle menu"
                >
                    <span class="ham-line"></span>
                    <span class="ham-line"></span>
                    <span class="ham-line"></span>
                </button>
            </div>

            <div class=move || if menu_open.get() { "nav-mobile-menu open" } else { "nav-mobile-menu" }>
                <A href="/" attr:class="nav-link" on:click=move |_| set_menu_open.set(false)>"~/about"</A>
                <A href="/projects" attr:class="nav-link" on:click=move |_| set_menu_open.set(false)>"~/projects"</A>
                <A href="/iot" attr:class="nav-link" on:click=move |_| set_menu_open.set(false)>"~/iot"</A>
                <A href="/contact" attr:class="nav-link" on:click=move |_| set_menu_open.set(false)>"~/contact"</A>
            </div>
        </nav>
    }
}