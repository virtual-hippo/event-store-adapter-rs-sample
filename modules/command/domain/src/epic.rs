use crate::backlog_item::BacklogItem;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epic {
    id: String,
    title: String,
    description: String,
    backlog_items: Vec<BacklogItem>,
}

impl Epic {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            backlog_items: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn backlog_items(&self) -> &[BacklogItem] {
        &self.backlog_items
    }

    pub fn add_backlog_item(&mut self, item: BacklogItem) {
        self.backlog_items.push(item);
    }

    pub fn remove_backlog_item(&mut self, id: &str) -> Option<BacklogItem> {
        if let Some(index) = self.backlog_items.iter().position(|item| item.id() == id) {
            Some(self.backlog_items.remove(index))
        } else {
            None
        }
    }

    pub fn progress(&self) -> f32 {
        if self.backlog_items.is_empty() {
            return 0.0;
        }

        let completed_items = self.backlog_items.iter().filter(|item| item.is_complete()).count();
        completed_items as f32 / self.backlog_items.len() as f32
    }
}
