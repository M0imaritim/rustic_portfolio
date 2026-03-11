use crate::data::{Project, ProjectStatus};
use leptos::prelude::*;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    let status_class = match project.status {
        ProjectStatus::Live => "badge badge-live",
        ProjectStatus::InProgress => "badge badge-wip",
        ProjectStatus::Demo => "badge badge-demo",
    }
    .to_string();

    let title = project.title; // &'static str — fine
    let desc = project.description; // &'static str — fine
    let tags = project.tags; // &'static [&'static str] — fine
    let live_url = project.live_url; // Option<&'static str> — fine
    let repo_url = project.repo_url; // Option<&'static str> — fine
    let status_label = project.status.label().to_string(); // borrow is gone after this

    view! {
        <article class="project-card">
            <div class="project-card-header">
                <div class="project-card-title-row">
                    <h3 class="project-card-title">{title}</h3>
                    <span class=status_class>{status_label}</span>
                </div>
                <p class="project-card-desc">{desc}</p>
            </div>
            <div class="project-card-tags">
                {tags.iter().map(|tag| view! {
                    <span class="tag">{*tag}</span>
                }).collect::<Vec<_>>()}
            </div>
            <div class="project-card-footer">
                <div class="project-card-links">
                    {live_url.map(|url| view! {
                        <a href=url target="_blank" rel="noopener" class="btn btn-primary">"↗ Live"</a>
                    })}
                    {repo_url.map(|url| view! {
                        <a href=url target="_blank" rel="noopener" class="btn btn-ghost">"{ } Code"</a>
                    })}
                </div>
            </div>
        </article>
    }
}
