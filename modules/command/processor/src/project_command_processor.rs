use event_store_adapter_rs::types::Event;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

use command_domain::project::{MemberId, MemberRole, Members, Project, ProjectError, ProjectId, ProjectName};
use command_domain::user::UserId;
use command_interface_adaptor_if::{ProjectRepository, ProjectRepositoryError};

#[derive(Error, Debug)]
pub enum CommandProcessError {
    #[error("Project not found.")]
    NotFoundError,
    #[error("ProjectRepositoryError: {0:?}")]
    RepositoryError(#[from] ProjectRepositoryError),
    #[error("ProjectError: {0:?}")]
    DomainLogicError(#[from] ProjectError),
}

pub struct ProjectCommandProcessor<TR: ProjectRepository> {
    project_repository: Arc<Mutex<TR>>,
}

impl<TR: ProjectRepository> ProjectCommandProcessor<TR> {
    pub fn new(project_repository: TR) -> Self {
        Self {
            project_repository: Arc::new(Mutex::new(project_repository)),
        }
    }

    pub async fn create_project(
        &mut self,
        name: ProjectName,
        executor_id: UserId,
    ) -> Result<ProjectId, CommandProcessError> {
        let mut repository_mg = self.project_repository.lock().await;

        let members = Members::new();
        let (project, project_event) = Project::new(name, members, executor_id);

        repository_mg
            .store(&project_event, &project)
            .await
            .map(|_| project_event.aggregate_id().clone())
            .map_err(CommandProcessError::RepositoryError)
    }

    pub async fn add_member(
        &mut self,
        project_id: ProjectId,
        user_id: UserId,
        role: MemberRole,
        executor_id: UserId,
    ) -> Result<ProjectId, CommandProcessError> {
        let mut repository_mg = self.project_repository.lock().await;

        let mut project = repository_mg
            .find_by_id(&project_id)
            .await
            .map_err(CommandProcessError::RepositoryError)?
            .ok_or(CommandProcessError::NotFoundError)?;

        let member_id = MemberId::new();
        let project_event = project
            .add_member(member_id, user_id, role, executor_id)
            .map_err(CommandProcessError::DomainLogicError)?;

        repository_mg
            .store(&project_event, &project)
            .await
            .map(|_| project_event.aggregate_id().clone())
            .map_err(CommandProcessError::RepositoryError)
    }

    pub async fn remove_member(
        &mut self,
        project_id: ProjectId,
        user_id: UserId,
        executor_id: UserId,
    ) -> Result<ProjectId, CommandProcessError> {
        let mut repository_mg = self.project_repository.lock().await;

        let mut project = repository_mg
            .find_by_id(&project_id)
            .await
            .map_err(CommandProcessError::RepositoryError)?
            .ok_or(CommandProcessError::NotFoundError)?;

        let project_event = project
            .remove_member(user_id, executor_id)
            .map_err(CommandProcessError::DomainLogicError)?;

        repository_mg
            .store(&project_event, &project)
            .await
            .map(|_| project_event.aggregate_id().clone())
            .map_err(CommandProcessError::RepositoryError)
    }

    pub async fn delete_project(
        &mut self,
        project_id: ProjectId,
        executor_id: UserId,
    ) -> Result<ProjectId, CommandProcessError> {
        let mut repository_mg = self.project_repository.lock().await;

        let mut project = repository_mg
            .find_by_id(&project_id)
            .await
            .map_err(CommandProcessError::RepositoryError)?
            .ok_or(CommandProcessError::NotFoundError)?;

        let project_event = project.delete(executor_id).map_err(CommandProcessError::DomainLogicError)?;

        repository_mg
            .store(&project_event, &project)
            .await
            .map(|_| project_event.aggregate_id().clone())
            .map_err(CommandProcessError::RepositoryError)
    }
}
