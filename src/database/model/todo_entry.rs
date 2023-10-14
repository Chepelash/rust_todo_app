use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct TodoEntry {
    id: String,
    pub priority: u32,
    pub title: String,
    pub description: String,
    pub language: String,
    pub done: bool,
    pub github_link: String,
}

impl Default for TodoEntry {
    fn default() -> Self {
        Self {
            id: Self::generate_id(),
            priority: (0),
            title: ("Enter title".to_string()),
            description: ("".to_string()),
            language: ("".to_string()),
            done: (false),
            github_link: ("".to_string()),
        }
    }
}

impl TodoEntry {
    fn generate_id() -> String {
        Utc::now().format("%d/%m/%Y-%H:%M:%S:%f").to_string()
    }
}
