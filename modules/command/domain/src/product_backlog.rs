use crate::backlog_item::BacklogItem;
use crate::types::Priority;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductBacklog {
    id: String,
    name: String,
    description: String,
    items: Vec<BacklogItem>,
}

impl ProductBacklog {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            items: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn items(&self) -> &[BacklogItem] {
        &self.items
    }

    pub fn add_item(&mut self, item: BacklogItem) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, id: &str) -> Option<BacklogItem> {
        if let Some(index) = self.items.iter().position(|item| item.id() == id) {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn prioritize_items(&mut self) {
        self.items.sort_by(|a, b| {
            // Sort by priority (High > Medium > Low)
            match (a.priority(), b.priority()) {
                (Priority::High, Priority::High) => std::cmp::Ordering::Equal,
                (Priority::High, _) => std::cmp::Ordering::Less,
                (_, Priority::High) => std::cmp::Ordering::Greater,
                (Priority::Medium, Priority::Medium) => std::cmp::Ordering::Equal,
                (Priority::Medium, _) => std::cmp::Ordering::Less,
                (_, Priority::Medium) => std::cmp::Ordering::Greater,
                (Priority::Low, Priority::Low) => std::cmp::Ordering::Equal,
            }
        });
    }
}
