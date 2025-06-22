use chrono::{DateTime, Utc};
use event_store_adapter_rs::types::Aggregate;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use ulid_generator_rs::ULIDError;

mod project_events;
mod project_id;
mod project_name;

pub use crate::project::project_events::{ProjectEvent, ProjectEventCreatedBody};
pub use crate::project::project_id::ProjectId;
pub use crate::project::project_name::ProjectName;

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error("invalid ULID format: {0}")]
    InvalidULID(#[from] ULIDError),
}

// Serialize, Deserialize はドメインモデルに実装しないようにしたい
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    id: ProjectId,
    deleted: bool,
    name: ProjectName,
    version: usize,
    seq_nr_counter: usize,
    last_updated_at: DateTime<Utc>,
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Aggregate for Project {
    type ID = ProjectId;

    fn id(&self) -> &Self::ID {
        &self.id
    }

    fn seq_nr(&self) -> usize {
        self.seq_nr_counter
    }

    fn version(&self) -> usize {
        self.version
    }

    fn set_version(&mut self, version: usize) {
        self.version = version;
    }

    fn last_updated_at(&self) -> &DateTime<Utc> {
        &self.last_updated_at
    }
}

impl Project {
    pub fn new(name: ProjectName) -> (Self, ProjectEvent) {
        let id = ProjectId::new();
        Self::from(id, false, name, 0, 1)
    }

    pub fn from(
        id: ProjectId,
        deleted: bool,
        name: ProjectName,
        seq_nr_counter: usize,
        version: usize,
    ) -> (Self, ProjectEvent) {
        let mut my_self = Self {
            id: id.clone(),
            deleted,
            name: name.clone(),
            seq_nr_counter,
            version,
            last_updated_at: Utc::now(),
        };
        my_self.seq_nr_counter += 1;
        let event = ProjectEvent::ProjectCreated(ProjectEventCreatedBody::new(
            id,
            my_self.seq_nr_counter,
            name,
        ));
        (my_self, event)
    }
}

#[cfg(test)]
mod tests {
    // TODO: implement tests for Project
}
