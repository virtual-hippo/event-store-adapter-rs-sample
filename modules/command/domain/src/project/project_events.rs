use chrono::{DateTime, Utc};
use event_store_adapter_rs::types::Event;
use serde::{Deserialize, Serialize};
use ulid_generator_rs::ULID;

use crate::helper::id_generate;
use crate::project::Member;
use crate::project::Members;
use crate::project::ProjectId;
use crate::project::ProjectName;
pub use crate::user::user_id::UserId;

pub type ProjectEventId = ULID;

/// プロジェクトに関するイベント
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ProjectEvent {
    /// プロジェクトが作成された
    ProjectCreated(ProjectEventCreatedBody),
    /// プロジェクトが削除された
    ProjectDeleted(ProjectEventDeletedBody),
    /// プロジェクトにメンバーが追加された
    ProjectMemberAdded(ProjectEventMemberAddedBody),
    /// プロジェクトのメンバーが削除された
    ProjectMemberRemoved(ProjectEventMemberRemovedBody),
}

impl Event for ProjectEvent {
    type AggregateID = ProjectId;
    type ID = ProjectEventId;

    fn id(&self) -> &ProjectEventId {
        match self {
            ProjectEvent::ProjectCreated(event) => &event.id,
            ProjectEvent::ProjectDeleted(event) => &event.id,
            ProjectEvent::ProjectMemberAdded(event) => &event.id,
            ProjectEvent::ProjectMemberRemoved(event) => &event.id,
        }
    }

    fn seq_nr(&self) -> usize {
        match self {
            ProjectEvent::ProjectCreated(event) => event.seq_nr,
            ProjectEvent::ProjectDeleted(event) => event.seq_nr,
            ProjectEvent::ProjectMemberAdded(event) => event.seq_nr,
            ProjectEvent::ProjectMemberRemoved(event) => event.seq_nr,
        }
    }

    fn aggregate_id(&self) -> &ProjectId {
        match self {
            ProjectEvent::ProjectCreated(event) => &event.aggregate_id,
            ProjectEvent::ProjectDeleted(event) => &event.aggregate_id,
            ProjectEvent::ProjectMemberAdded(event) => &event.aggregate_id,
            ProjectEvent::ProjectMemberRemoved(event) => &event.aggregate_id,
        }
    }

    fn occurred_at(&self) -> &DateTime<Utc> {
        match self {
            ProjectEvent::ProjectCreated(event) => &event.occurred_at,
            ProjectEvent::ProjectDeleted(event) => &event.occurred_at,
            ProjectEvent::ProjectMemberAdded(event) => &event.occurred_at,
            ProjectEvent::ProjectMemberRemoved(event) => &event.occurred_at,
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
    pub members: Members,
    pub executor_id: UserId,
    pub occurred_at: DateTime<Utc>,
}

impl ProjectEventCreatedBody {
    pub fn new(
        aggregate_id: ProjectId,
        seq_nr: usize,
        name: ProjectName,
        members: Members,
        executor_id: UserId,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        let id = id_generate();
        Self {
            id,
            aggregate_id,
            seq_nr,
            name,
            members,
            executor_id,
            occurred_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEventDeletedBody {
    pub id: ProjectEventId,
    pub aggregate_id: ProjectId,
    pub seq_nr: usize,
    pub executor_id: UserId,
    pub occurred_at: DateTime<Utc>,
}

impl ProjectEventDeletedBody {
    pub fn new(aggregate_id: ProjectId, seq_nr: usize, executor_id: UserId, occurred_at: DateTime<Utc>) -> Self {
        let id = id_generate();
        Self {
            id,
            aggregate_id,
            seq_nr,
            executor_id,
            occurred_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEventMemberAddedBody {
    pub id: ProjectEventId,
    pub aggregate_id: ProjectId,
    pub seq_nr: usize,
    pub member: Member,
    pub executor_id: UserId,
    pub occurred_at: DateTime<Utc>,
}

impl ProjectEventMemberAddedBody {
    pub fn new(
        aggregate_id: ProjectId,
        seq_nr: usize,
        member: Member,
        executor_id: UserId,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        let id = id_generate();
        Self {
            id,
            aggregate_id,
            seq_nr,
            member,
            executor_id,
            occurred_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectEventMemberRemovedBody {
    pub(crate) id: ProjectEventId,
    pub aggregate_id: ProjectId,
    pub(crate) seq_nr: usize,
    pub user_id: UserId,
    pub(crate) executor_id: UserId,
    pub(crate) occurred_at: DateTime<Utc>,
}

impl ProjectEventMemberRemovedBody {
    pub fn new(
        aggregate_id: ProjectId,
        seq_nr: usize,
        user_id: UserId,
        executor_id: UserId,
        occurred_at: DateTime<Utc>,
    ) -> Self {
        let id = id_generate();
        Self {
            id,
            aggregate_id,
            seq_nr,
            user_id,
            executor_id,
            occurred_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::project::project_events::{ProjectEvent, ProjectEventCreatedBody};
    use crate::project::{Members, ProjectId, ProjectName};
    use crate::user::user_id::UserId;
    use chrono::Utc;
    use event_store_adapter_rs::types::Event;

    #[test]
    fn test_to_json() {
        let project_id = ProjectId::default();
        let project = ProjectName::new("test").unwrap();
        let user_id = UserId::default();
        let now = Utc::now();
        let event = ProjectEvent::ProjectCreated(ProjectEventCreatedBody::new(
            project_id,
            1usize,
            project,
            Members::default(),
            user_id,
            now,
        ));
        let json = serde_json::to_string(&event);
        let _occurred_at = event.occurred_at().timestamp_millis();
        println!("{}", json.unwrap());
    }
}
