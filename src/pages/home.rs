use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::terminal::TerminalHero;
use crate::components::skills::SkillsGrid;
use crate::components::project_card::ProjectCard;
use crate::data::get_projects;

#[component]
pub fn HomePage() -> impl IntoView {
    let featured: Vec<_> = get_projects().into_iter().filter(|p| p.featured).collect();

    view! {
        <div class="page home-page">

            // ── Hero ──────────────────────────────────────────
            <section class="hero">
                <div class="hero-text">
                    <p class="hero-eyebrow">"// Hello, world."</p>
                    <h1 class="hero-name">
                        "Langat"
                        <br/>
                        "Moimaritim"
                    </h1>
                    <p class="hero-role">
                        "Full Stack Engineer"
                        <span class="hero-sep">" · "</span>
                        <span class="hero-rust">"Rust in Progress"</span>
                        <span class="hero-flag">" 🇰🇪"</span>
                    </p>
                    <div class="hero-cta">
                        <A href="/projects" attr:class="btn btn-primary">"View Projects"</A>
                        <A href="/contact" attr:class="btn btn-ghost">"Get in Touch"</A>
                    </div>
                </div>
                <div class="hero-terminal">
                    <TerminalHero/>
                </div>
            </section>

            // ── About ─────────────────────────────────────────
            <section class="section about-section">
                <h2 class="section-title">
                    <span class="section-num">"01"</span>
                    "About"
                </h2>
                <div class="about-grid">
                    <div class="about-text">
                        <p>
                            "I solve problems with code. That's what drives me every day as a backend developer "
                            "who's found his groove in Django and PostgreSQL — and who's now going all-in on Rust."
                        </p>
                        <p>
                            "What started as curiosity about how things work under the hood has grown into "
                            "expertise in building systems that actually last: e-commerce platforms handling "
                            "real M-PESA transactions, high-traffic news sites serving hundreds of thousands "
                            "of readers, and peer-to-peer platforms built for Nairobi."
                        </p>
                        <p>
                            "Rust is my current deep dive. I'm building real projects with Leptos and Axum — "
                            "not toy apps, but production-grade tools. The borrow checker is humbling; "
                            "the performance and safety guarantees make it worth every compile error."
                        </p>
                        <p class="philosophy">
                            <span class="quote-mark">""</span>
                            "If it's worth building, it's worth building right."
                            <span class="quote-mark">""</span>
                        </p>
                        <div class="about-links">
                            <a
                                href="https://drive.google.com/file/d/1XddbL9xZCQ8dg2EbGjRwXQm1GR5X4_oP/view"
                                target="_blank"
                                rel="noopener"
                                class="cert-link"
                            >
                                "↗ Full-Stack Certification"
                            </a>
                            <a
                                href="https://github.com/M0imaritim"
                                target="_blank"
                                rel="noopener"
                                class="cert-link"
                            >
                                "↗ GitHub"
                            </a>
                            <a
                                href="https://www.linkedin.com/in/langat-moimaritim/"
                                target="_blank"
                                rel="noopener"
                                class="cert-link"
                            >
                                "↗ LinkedIn"
                            </a>
                        </div>
                    </div>
                    <div class="about-stats">
                        <div class="stat-card">
                            <span class="stat-num">"3+"</span>
                            <span class="stat-label">"Years building\nproduction systems"</span>
                        </div>
                        <div class="stat-card">
                            <span class="stat-num">"2"</span>
                            <span class="stat-label">"High-traffic Kenyan\nnews platforms"</span>
                        </div>
                        <div class="stat-card rust-stat">
                            <span class="stat-num">"🦀"</span>
                            <span class="stat-label">"Rust: active\ndeep dive"</span>
                        </div>
                        <div class="stat-card">
                            <span class="stat-num">"NBI"</span>
                            <span class="stat-label">"Based in\nNairobi, Kenya"</span>
                        </div>
                    </div>
                </div>
            </section>

            // ── Featured Projects ─────────────────────────────
            <section class="section projects-section">
                <div class="section-header">
                    <h2 class="section-title">
                        <span class="section-num">"02"</span>
                        "Featured Work"
                    </h2>
                    <A href="/projects" attr:class="section-link">"View all →"</A>
                </div>
                <div class="projects-grid">
                    {featured.into_iter().map(|p| view! { <ProjectCard project=p/> }).collect::<Vec<_>>()}
                </div>
            </section>

            // ── Skills ────────────────────────────────────────
            <section class="section skills-section">
                <h2 class="section-title">
                    <span class="section-num">"03"</span>
                    "Skills & Stack"
                </h2>
                <p class="section-subtitle">
                    "Honest levels — no inflated bars."
                    <span class="rust-orange">" Rust levels reflect where I am today, not where I'm headed."</span>
                </p>
                <SkillsGrid/>
            </section>

        </div>
    }
}