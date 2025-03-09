use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptanceCriterion {
    id: String,
    description: String,
    is_satisfied: bool,
}

impl AcceptanceCriterion {
    pub fn new(description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            description,
            is_satisfied: false,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_satisfied(&self) -> bool {
        self.is_satisfied
    }

    pub fn mark_satisfied(&mut self) {
        self.is_satisfied = true;
    }

    pub fn mark_unsatisfied(&mut self) {
        self.is_satisfied = false;
    }
}
