use chrono::Utc;
use serde::{Deserialize, Serialize};
use surreal_id::NewId;
use surrealdb::{opt::RecordId, sql::Id};

use crate::database::repository::TABLE;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TodoEntry {
    pub id: TodoRecordId,
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
    fn generate_id() -> TodoRecordId {
        TodoRecordId::new(Utc::now().format("%d%m%Y%H%M%S%f").to_string()).unwrap()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TodoRecordId(RecordId);

impl NewId for TodoRecordId {
    const TABLE: &'static str = TABLE;

    fn from_inner_id<T: Into<Id>>(inner_id: T) -> Self {
        TodoRecordId(RecordId {
            tb: Self::TABLE.to_string(),
            id: inner_id.into(),
        })
    }

    fn get_inner_string(&self) -> String {
        self.0.id.to_string()
    }
}
