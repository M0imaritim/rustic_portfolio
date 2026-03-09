use leptos::prelude::*;
use crate::data::{Skill, SkillCategory};

#[component]
pub fn SkillBar(skill: Skill) -> impl IntoView {
    let cat_class = match skill.category {
        SkillCategory::Rust => "skill-cat-rust",
        SkillCategory::Backend => "skill-cat-backend",
        SkillCategory::Frontend => "skill-cat-frontend",
        SkillCategory::Infrastructure => "skill-cat-infra",
    };

    let width = format!("{}%", skill.level);

    view! {
        <div class="skill-item">
            <div class="skill-meta">
                <span class="skill-name">{skill.name}</span>
                <span class="skill-note">{skill.note}</span>
            </div>
            <div class="skill-bar-track">
                <div
                    class=format!("skill-bar-fill {}", cat_class)
                    style=format!("width: {}", width)
                ></div>
            </div>
        </div>
    }
}

#[component]
pub fn SkillsGrid() -> impl IntoView {
    use crate::data::get_skills;

    let all_skills = get_skills();

    let rust_skills: Vec<_> = all_skills.iter().filter(|s| s.category == SkillCategory::Rust).cloned().collect();
    let backend_skills: Vec<_> = all_skills.iter().filter(|s| s.category == SkillCategory::Backend).cloned().collect();
    let frontend_skills: Vec<_> = all_skills.iter().filter(|s| s.category == SkillCategory::Frontend).cloned().collect();
    let infra_skills: Vec<_> = all_skills.iter().filter(|s| s.category == SkillCategory::Infrastructure).cloned().collect();

    view! {
        <div class="skills-grid">
            <div class="skills-group">
                <h4 class="skills-group-title rust-title">"🦀 Rust Ecosystem"</h4>
                <div class="skills-list">
                    {rust_skills.into_iter().map(|s| view! { <SkillBar skill=s/> }).collect::<Vec<_>>()}
                </div>
            </div>
            <div class="skills-group">
                <h4 class="skills-group-title">"⚙ Backend"</h4>
                <div class="skills-list">
                    {backend_skills.into_iter().map(|s| view! { <SkillBar skill=s/> }).collect::<Vec<_>>()}
                </div>
            </div>
            <div class="skills-group">
                <h4 class="skills-group-title">"◈ Frontend"</h4>
                <div class="skills-list">
                    {frontend_skills.into_iter().map(|s| view! { <SkillBar skill=s/> }).collect::<Vec<_>>()}
                </div>
            </div>
            <div class="skills-group">
                <h4 class="skills-group-title">"▣ Infrastructure"</h4>
                <div class="skills-list">
                    {infra_skills.into_iter().map(|s| view! { <SkillBar skill=s/> }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}