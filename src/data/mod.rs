// use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub slug: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub long_description: &'static str,
    pub tags: &'static [&'static str],
    pub live_url: Option<&'static str>,
    pub repo_url: Option<&'static str>,
    pub image: &'static str,
    pub featured: bool,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectStatus {
    Live,
    InProgress,
    Demo,
}

impl ProjectStatus {
    pub fn label(&self) -> &str {
        match self {
            Self::Live => "LIVE",
            Self::InProgress => "IN PROGRESS",
            Self::Demo => "DEMO",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: &'static str,
    pub level: u8, // 0–100
    pub category: SkillCategory,
    pub note: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkillCategory {
    Rust,
    Backend,
    Frontend,
    Infrastructure,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            slug: "runsafi",
            title: "RunSafi Errands",
            description: "P2P errand platform for Nairobi with M-PESA integration.",
            long_description: "Full-stack peer-to-peer errands platform built for Nairobi. Features M-PESA payment integration, JWT auth, task & squad management. Built with Leptos + Axum on the backend and WASM on the frontend.",
            tags: &["Rust", "Leptos", "Axum", "PostgreSQL", "M-PESA", "JWT"],
            live_url: None,
            repo_url: Some("https://github.com/M0imaritim/e-boy"),
            image: "/public/images/runsafi.png",
            featured: true,
            status: ProjectStatus::InProgress,
        },
        Project {
            slug: "the-star",
            title: "The Star Kenya",
            description: "High-traffic Kenyan news platform — dev team contributor.",
            long_description: "Leading Kenyan digital news platform. Part of the development team responsible for building and maintaining this high-traffic site serving hundreds of thousands of daily readers.",
            tags: &["Next.js", "TypeScript", "React", "Tailwind"],
            live_url: Some("https://the-star.co.ke"),
            repo_url: None,
            image: "/public/images/the-star.png",
            featured: true,
            status: ProjectStatus::Live,
        },
        Project {
            slug: "mpasho",
            title: "Mpasho",
            description: "Kenya's premier entertainment & celebrity news platform.",
            long_description: "Kenya's most popular entertainment news website. Part of the dev team building and maintaining this high-traffic lifestyle platform known for its vibrant content.",
            tags: &["Next.js", "TypeScript", "React"],
            live_url: Some("https://mpasho.co.ke"),
            repo_url: None,
            image: "/public/images/mpasho.png",
            featured: false,
            status: ProjectStatus::Live,
        },
        Project {
            slug: "radio-jambo",
            title: "Radio Jambo",
            description: "Digital platform for one of Kenya's most popular Swahili radio stations.",
            long_description: "Official digital home for Radio Jambo — live streaming, news, and entertainment for their online Swahili-speaking audience.",
            tags: &["Next.js", "React", "Streaming"],
            live_url: Some("https://radiojambo.co.ke"),
            repo_url: None,
            image: "/public/images/radio-jambo.png",
            featured: false,
            status: ProjectStatus::Live,
        },
        Project {
            slug: "patient-management",
            title: "Patient Management System",
            description: "Web-based system for managing patient records.",
            long_description: "Full-stack patient records management system. Django REST API backend with a Next.js frontend. Handles patient intake, record management, and appointment workflows.",
            tags: &["Django", "Next.js", "PostgreSQL", "REST API"],
            live_url: None,
            repo_url: Some("https://github.com/M0imaritim/patient-management-system"),
            image: "/public/images/pms.png",
            featured: false,
            status: ProjectStatus::InProgress,
        },
        Project {
            slug: "maze-game",
            title: "3D Maze Game",
            description: "Real-time 3D raycasting maze explorer built in C with SDL2.",
            long_description: "A pseudo-3D maze game using raycasting rendering — the same technique used in classic Wolfenstein 3D. Built in C with SDL2 for rendering. Demonstrates low-level graphics programming from scratch.",
            tags: &["C", "SDL2", "Raycasting", "Graphics"],
            live_url: Some("https://www.youtube.com/watch?v=xk7hwCbyz9s"),
            repo_url: Some("https://github.com/M0imaritim/Maze"),
            image: "/public/images/maze.png",
            featured: true,
            status: ProjectStatus::Demo,
        },
    ]
}

pub fn get_skills() -> Vec<Skill> {
    vec![
        // Rust
        Skill {
            name: "Rust",
            level: 55,
            category: SkillCategory::Rust,
            note: "Actively deepening",
        },
        Skill {
            name: "Leptos",
            level: 50,
            category: SkillCategory::Rust,
            note: "Full-stack WASM/SSR",
        },
        Skill {
            name: "Axum",
            level: 52,
            category: SkillCategory::Rust,
            note: "REST + server-side",
        },
        Skill {
            name: "Ownership & Lifetimes",
            level: 48,
            category: SkillCategory::Rust,
            note: "In progress",
        },
        // Backend
        Skill {
            name: "Django",
            level: 82,
            category: SkillCategory::Backend,
            note: "Production grade",
        },
        Skill {
            name: "PostgreSQL",
            level: 75,
            category: SkillCategory::Backend,
            note: "Primary DB",
        },
        Skill {
            name: "Django REST Framework",
            level: 80,
            category: SkillCategory::Backend,
            note: "API design",
        },
        Skill {
            name: "Python",
            level: 80,
            category: SkillCategory::Backend,
            note: "Core language",
        },
        // Frontend
        Skill {
            name: "Next.js",
            level: 75,
            category: SkillCategory::Frontend,
            note: "App Router",
        },
        Skill {
            name: "React",
            level: 72,
            category: SkillCategory::Frontend,
            note: "Hooks + Server Components",
        },
        Skill {
            name: "TypeScript",
            level: 68,
            category: SkillCategory::Frontend,
            note: "Typed JS",
        },
        Skill {
            name: "Tailwind CSS",
            level: 80,
            category: SkillCategory::Frontend,
            note: "Preferred styling",
        },
        // Infrastructure
        Skill {
            name: "Docker",
            level: 65,
            category: SkillCategory::Infrastructure,
            note: "Containerised deploys",
        },
        Skill {
            name: "Railway / Vercel",
            level: 70,
            category: SkillCategory::Infrastructure,
            note: "Cloud hosting",
        },
        Skill {
            name: "Git",
            level: 78,
            category: SkillCategory::Infrastructure,
            note: "Daily workflow",
        },
    ]
}
