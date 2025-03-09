use crate::acceptance_criterion::AcceptanceCriterion;
use crate::types::{Priority, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacklogItem {
    id: String,
    title: String,
    description: String,
    priority: Priority,
    status: Status,
    story_points: Option<u8>,
    acceptance_criteria: Vec<AcceptanceCriterion>,
}

impl BacklogItem {
    pub fn new(title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            description,
            priority: Priority::Medium,
            status: Status::Todo,
            story_points: None,
            acceptance_criteria: Vec::new(),
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

    pub fn priority(&self) -> &Priority {
        &self.priority
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn update_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn story_points(&self) -> Option<u8> {
        self.story_points
    }

    pub fn set_story_points(&mut self, points: u8) {
        self.story_points = Some(points);
    }

    pub fn acceptance_criteria(&self) -> &[AcceptanceCriterion] {
        &self.acceptance_criteria
    }

    pub fn add_acceptance_criterion(&mut self, criterion: AcceptanceCriterion) {
        self.acceptance_criteria.push(criterion);
    }

    pub fn remove_acceptance_criterion(&mut self, id: &str) -> Option<AcceptanceCriterion> {
        if let Some(index) = self.acceptance_criteria.iter().position(|c| c.id() == id) {
            Some(self.acceptance_criteria.remove(index))
        } else {
            None
        }
    }

    pub fn is_complete(&self) -> bool {
        if self.acceptance_criteria.is_empty() {
            return self.status == Status::Done;
        }

        self.status == Status::Done && self.acceptance_criteria.iter().all(|c| c.is_satisfied())
    }
}
