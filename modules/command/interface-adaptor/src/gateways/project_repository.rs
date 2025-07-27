use event_store_adapter_rs::types::{Aggregate, Event, EventStore};
use std::collections::{HashMap, VecDeque};

use command_domain::project::ProjectEvent;
use command_domain::project::{Project, ProjectId};
use command_interface_adaptor_if::{ProjectRepository, ProjectRepositoryError};

#[derive(Debug, Clone)]
pub struct MockProjectRepository {
    events: HashMap<ProjectId, VecDeque<ProjectEvent>>,
    snapshot: HashMap<ProjectId, Option<Project>>,
}

impl MockProjectRepository {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            snapshot: HashMap::new(),
        }
    }
}

impl Default for MockProjectRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl ProjectRepository for MockProjectRepository {
    async fn store(&mut self, event: &ProjectEvent, snapshot: &Project) -> Result<(), ProjectRepositoryError> {
        self.events
            .entry(event.aggregate_id().clone())
            .or_default()
            .push_back(event.clone());

        *self
            .snapshot
            .entry(event.aggregate_id().clone())
            .or_insert(Some(snapshot.clone())) = Some(snapshot.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &ProjectId) -> Result<Option<Project>, ProjectRepositoryError> {
        let events = self.events.get(id).unwrap().iter().cloned().collect::<Vec<_>>();
        let snapshot_opt = self.snapshot.get(id).unwrap().clone();
        if let Some(snapshot) = snapshot_opt {
            let result = Project::replay(&events, snapshot);
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone)]
pub struct AwsDynamoDbProjectRepository<ES: EventStore<AID = ProjectId, AG = Project, EV = ProjectEvent>> {
    event_store: ES,
    snapshot_interval: usize,
}

unsafe impl<ES: EventStore<AID = ProjectId, AG = Project, EV = ProjectEvent>> Sync
    for AwsDynamoDbProjectRepository<ES>
{
}

unsafe impl<ES: EventStore<AID = ProjectId, AG = Project, EV = ProjectEvent>> Send
    for AwsDynamoDbProjectRepository<ES>
{
}

impl<ES: EventStore<AID = ProjectId, AG = Project, EV = ProjectEvent>> AwsDynamoDbProjectRepository<ES> {
    pub fn new(event_store: ES, snapshot_interval: usize) -> Self {
        Self { event_store, snapshot_interval }
    }

    /// スナップショットを永続化するかどうかを判定する。
    ///
    /// # 引数
    /// - `snapshot_interval` - スナップショットを永続化する間隔
    /// - `created` - プロジェクトが作成されたかどうか
    /// - `project` - プロジェクト
    ///
    /// # 戻り値
    /// スナップショットを永続化する場合は `Some` 、そうでない場合は `None` 。
    fn resolve_snapshot(snapshot_interval: usize, created: bool, project: &Project) -> Option<&Project> {
        if created || project.seq_nr() % snapshot_interval == 0 {
            Some(project)
        } else {
            None
        }
    }
}

#[async_trait::async_trait]
impl<ES: EventStore<AID = ProjectId, AG = Project, EV = ProjectEvent>> ProjectRepository
    for AwsDynamoDbProjectRepository<ES>
{
    async fn store(&mut self, event: &ProjectEvent, snapshot: &Project) -> Result<(), ProjectRepositoryError> {
        let result = match Self::resolve_snapshot(self.snapshot_interval, event.is_created(), snapshot) {
            Some(snapshot) => self.event_store.persist_event_and_snapshot(event, snapshot).await,
            None => self.event_store.persist_event(event, snapshot.version()).await,
        };
        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(ProjectRepositoryError::StoreError(snapshot.clone(), error)),
        }
    }

    async fn find_by_id(&self, id: &ProjectId) -> Result<Option<Project>, ProjectRepositoryError> {
        let snapshot_opt = self.event_store.get_latest_snapshot_by_id(id).await;
        match snapshot_opt {
            Ok(None) => Ok(None),
            Ok(Some(snapshot)) => {
                let events = self.event_store.get_events_by_id_since_seq_nr(id, snapshot.seq_nr()).await;
                match events {
                    Ok(events) => {
                        let result = Project::replay(&events, snapshot.clone());
                        Ok(Some(result))
                    },
                    Err(error) => Err(ProjectRepositoryError::FindByIdError(id.clone(), error)),
                }
            },
            Err(error) => Err(ProjectRepositoryError::FindByIdError(id.clone(), error)),
        }
    }
}
