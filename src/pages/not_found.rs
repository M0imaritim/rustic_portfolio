use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="page not-found-page">
            <div class="not-found-terminal">
                <div class="terminal-bar">
                    <span class="terminal-dot red"></span>
                    <span class="terminal-dot yellow"></span>
                    <span class="terminal-dot green"></span>
                    <span class="terminal-title">"error"</span>
                </div>
                <div class="terminal-body">
                    <div class="terminal-line output">
                        <span class="dim">"error["</span>
                        <span class="rust-orange">"E0404"</span>
                        <span class="dim">"]:"</span>
                        <span class="accent">" cannot find page in this scope"</span>
                    </div>
                    <div class="terminal-line output">
                        <span class="dim">"  --> "</span>
                        <span class="accent">"src/routes/404.rs:1:1"</span>
                    </div>
                    <div class="terminal-line output">
                        <span class="dim">"   |"</span>
                    </div>
                    <div class="terminal-line output">
                        <span class="dim">" 1 | "</span>
                        <span class="yellow-text">"use browser::url::this_page;"</span>
                    </div>
                    <div class="terminal-line output">
                        <span class="dim">"   | "</span>
                        <span class="rust-orange">"^^^^^^^^^ not found in `routes`"</span>
                    </div>
                </div>
            </div>
            <h1 class="not-found-title">"404"</h1>
            <p class="not-found-msg">"Page not found."</p>
            <A href="/" attr:class="btn btn-primary">"← Back to safety"</A>
        </div>
    }
}