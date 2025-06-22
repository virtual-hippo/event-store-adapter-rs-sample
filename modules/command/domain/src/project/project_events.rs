use chrono::{DateTime, Utc};
use event_store_adapter_rs::types::Event;
use serde::{Deserialize, Serialize};
use ulid_generator_rs::ULID;

use crate::helper::id_generate;
use crate::project::ProjectId;
use crate::project::ProjectName;

pub type ProjectEventId = ULID;

/// プロジェクトに関するイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ProjectEvent {
    /// プロジェクトが作成された
    ProjectCreated(ProjectEventCreatedBody),
    /// グループチャットが削除された
    ProjectDeleted(ProjectEventDeletedBody),
}

impl Event for ProjectEvent {
    type AggregateID = ProjectId;
    type ID = ProjectEventId;

    fn id(&self) -> &ProjectEventId {
        match self {
            ProjectEvent::ProjectCreated(event) => &event.id,
            ProjectEvent::ProjectDeleted(event) => &event.id,
        }
    }

    fn seq_nr(&self) -> usize {
        match self {
            ProjectEvent::ProjectCreated(event) => event.seq_nr,
            ProjectEvent::ProjectDeleted(event) => event.seq_nr,
        }
    }

    fn aggregate_id(&self) -> &ProjectId {
        match self {
            ProjectEvent::ProjectCreated(event) => &event.aggregate_id,
            ProjectEvent::ProjectDeleted(event) => &event.aggregate_id,
        }
    }

    fn occurred_at(&self) -> &DateTime<Utc> {
        match self {
            ProjectEvent::ProjectCreated(event) => &event.occurred_at,
            ProjectEvent::ProjectDeleted(event) => &event.occurred_at,
        }
    }

    fn is_created(&self) -> bool {
        match self {
            ProjectEvent::ProjectCreated(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEventCreatedBody {
    pub id: ProjectEventId,
    pub aggregate_id: ProjectId,
    pub seq_nr: usize,
    pub name: ProjectName,
    pub occurred_at: DateTime<Utc>,
}

impl ProjectEventCreatedBody {
    pub fn new(aggregate_id: ProjectId, seq_nr: usize, name: ProjectName) -> Self {
        let id = id_generate();
        let occurred_at = Utc::now();
        Self {
            id,
            aggregate_id,
            seq_nr,
            name,
            occurred_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEventDeletedBody {
    pub id: ProjectEventId,
    pub aggregate_id: ProjectId,
    pub seq_nr: usize,
    pub occurred_at: DateTime<Utc>,
}

impl ProjectEventDeletedBody {
    pub fn new(aggregate_id: ProjectId, seq_nr: usize) -> Self {
        let id = id_generate();
        let occurred_at = Utc::now();
        Self { id, aggregate_id, seq_nr, occurred_at }
    }
}

#[cfg(test)]
mod tests {
    use crate::project::project_events::{ProjectEvent, ProjectEventCreatedBody};
    use crate::project::{ProjectId, ProjectName};
    use event_store_adapter_rs::types::Event;

    #[test]
    fn test_to_json() {
        let project_id = ProjectId::new();
        let project = ProjectName::new("test").unwrap();
        let event = ProjectEvent::ProjectCreated(ProjectEventCreatedBody::new(project_id, 1usize, project));
        let json = serde_json::to_string(&event);
        let _occurred_at = event.occurred_at().timestamp_millis();
        println!("{}", json.unwrap());
    }
}
