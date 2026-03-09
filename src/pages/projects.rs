use leptos::prelude::*;
use crate::components::project_card::ProjectCard;
use crate::data::{get_projects, ProjectStatus};

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let (filter, set_filter) = signal("all".to_string());

    let all_projects = get_projects();

    let filtered = move || {
        let f = filter.get();
        all_projects.iter().filter(|p| match f.as_str() {
            "live" => p.status == ProjectStatus::Live,
            "wip" => p.status == ProjectStatus::InProgress,
            "rust" => p.tags.contains(&"Rust") || p.tags.contains(&"Leptos") || p.tags.contains(&"Axum"),
            _ => true,
        }).cloned().collect::<Vec<_>>()
    };

    view! {
        <div class="page projects-page">
            <div class="page-header">
                <p class="page-eyebrow">"~/projects"</p>
                <h1 class="page-title">"Things I've Built"</h1>
                <p class="page-subtitle">
                    "Production work, personal projects, and the Rust experiments that keep me up at night."
                </p>
            </div>

            <div class="filter-bar">
                {["all", "live", "wip", "rust"].iter().map(|f| {
                    let f_str = f.to_string();
                    let label = match *f {
                        "all" => "All",
                        "live" => "Live",
                        "wip" => "In Progress",
                        "rust" => "🦀 Rust",
                        _ => f,
                    };
                    view! {
                        <button
                            class=move || if filter.get() == f_str { "filter-btn active" } else { "filter-btn" }
                            on:click={
                                let f_str = f.to_string();
                                move |_| set_filter.set(f_str.clone())
                            }
                        >
                            {label}
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>

            <div class="projects-grid">
                {move || filtered().into_iter().map(|p| view! { <ProjectCard project=p/> }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}