use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationSection {
    pub title: String,
    pub expanded: bool,
    pub items: Vec<NavigationItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationItem {
    pub title: String,
    pub path: String,
    pub description: Option<String>,
}

pub fn get_navigation_items() -> Vec<NavigationSection> {
    vec![
        NavigationSection {
            title: "Getting Started".to_string(),
            expanded: true,
            items: vec![NavigationItem {
                title: "Introduction".to_string(),
                path: "/".to_string(),
                description: Some("Welcome to my portfolio".to_string()),
            }],
        },
        NavigationSection {
            title: "Projects".to_string(),
            expanded: false,
            items: vec![
                NavigationItem {
                    title: "Overview".to_string(),
                    path: "/projects".to_string(),
                    description: None,
                },
                NavigationItem {
                    title: "Web Applications".to_string(),
                    path: "/projects/web".to_string(),
                    description: None,
                },
                NavigationItem {
                    title: "CLI Tools".to_string(),
                    path: "/projects/cli".to_string(),
                    description: None,
                },
                NavigationItem {
                    title: "System Programming".to_string(),
                    path: "/projects/systems".to_string(),
                    description: None,
                },
            ],
        },
        NavigationSection {
            title: "Cheatsheets".to_string(),
            expanded: false,
            items: vec![
                NavigationItem {
                    title: "Git Commands".to_string(),
                    path: "/cheatsheets/git".to_string(),
                    description: Some("Essential Git workflows".to_string()),
                },
                NavigationItem {
                    title: "Docker".to_string(),
                    path: "/cheatsheets/docker".to_string(),
                    description: Some("Container management".to_string()),
                },
                NavigationItem {
                    title: "Rust".to_string(),
                    path: "/cheatsheets/rust".to_string(),
                    description: Some("Cargo and Rust tooling".to_string()),
                },
                NavigationItem {
                    title: "Linux/Unix".to_string(),
                    path: "/cheatsheets/linux".to_string(),
                    description: Some("System administration".to_string()),
                },
            ],
        },
    ]
}
