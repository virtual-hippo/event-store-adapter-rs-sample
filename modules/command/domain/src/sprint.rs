use crate::backlog_item::BacklogItem;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sprint {
    id: String,
    name: String,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    items: Vec<BacklogItem>,
}

impl Sprint {
    pub fn new(name: String, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            start_date,
            end_date,
            items: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn start_date(&self) -> &DateTime<Utc> {
        &self.start_date
    }

    pub fn end_date(&self) -> &DateTime<Utc> {
        &self.end_date
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

    pub fn is_active(&self, current_date: &DateTime<Utc>) -> bool {
        &self.start_date <= current_date && current_date <= &self.end_date
    }
}
