use leptos::prelude::*;

#[component]
pub fn TerminalHero() -> impl IntoView {
    view! {
        <div class="terminal">
            <div class="terminal-bar">
                <span class="terminal-dot red"></span>
                <span class="terminal-dot yellow"></span>
                <span class="terminal-dot green"></span>
                <span class="terminal-title">"langat@portfolio ~ zsh"</span>
            </div>
            <div class="terminal-body">
                <div class="terminal-line">
                    <span class="prompt">"langat@portfolio"</span>
                    <span class="prompt-sep">" :~$ "</span>
                    <span class="cmd">"cargo run --release"</span>
                </div>
                <div class="terminal-line output">
                    <span class="dim">"   Compiling "</span>
                    <span class="accent">"portfolio"</span>
                    <span class="dim">" v0.1.0"</span>
                </div>
                <div class="terminal-line output">
                    <span class="dim">"    Finished "</span>
                    <span class="green-text">"release"</span>
                    <span class="dim">" [optimized] target(s)"</span>
                </div>
                <div class="terminal-line output">
                    <span class="dim">"     Running "</span>
                    <span class="accent">"./langat_moimaritim"</span>
                </div>
                <div class="terminal-line output typed-line">
                    <span class="green-text">"🦀 Full Stack Engineer. Nairobi, Kenya."</span>
                </div>
                <div class="terminal-line output">
                    <span class="dim">"     Stack:   "</span>
                    <span class="rust-orange">"Rust"</span>
                    <span class="dim">" | Django | Next.js | PostgreSQL"</span>
                </div>
                <div class="terminal-line output">
                    <span class="dim">"    Status:   "</span>
                    <span class="yellow-text">"diving deep into Rust"</span>
                </div>
                <div class="terminal-line">
                    <span class="prompt">"langat@portfolio"</span>
                    <span class="prompt-sep">" :~$ "</span>
                    <span class="cursor-block">"█"</span>
                </div>
            </div>
        </div>
    }
}
