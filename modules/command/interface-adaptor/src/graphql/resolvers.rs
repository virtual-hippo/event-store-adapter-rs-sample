use async_graphql::{Context, Error, ErrorExtensions, FieldResult, Object};
use event_store_adapter_rs::types::EventStoreWriteError;
use std::str::FromStr;

use command_domain::project::{MemberRole, ProjectId, ProjectName};
use command_domain::user::UserId;
use command_interface_adaptor_if::ProjectRepositoryError;
use command_processor::project_command_processor::CommandProcessError;

use crate::gateways::project_repository::AwsDynamoDbProjectRepository;
use crate::graphql::inputs::{AddMemberInput, CreateProjectInput, DeleteProjectInput, RemoveMemberInput};
use crate::graphql::outputs::ProjectOut;
use crate::graphql::{ES, MutationRoot, ServiceContext};

#[Object]
impl MutationRoot {
    async fn create_project(&self, ctx: &Context<'_>, input: CreateProjectInput) -> FieldResult<ProjectOut> {
        let service_ctx = ctx.data::<ServiceContext<AwsDynamoDbProjectRepository<ES>>>().unwrap();

        let project_name = validate_project_name(&input.name)?;
        let executor_id = validate_user_id(&input.executor_id)?;

        let mut processor = service_ctx.project_command_processor.lock().await;
        processor
            .create_project(project_name, executor_id)
            .await
            .map(|project_id| ProjectOut::new(project_id.to_string()))
            .map_err(error_handling)
    }

    async fn delete_project(&self, ctx: &Context<'_>, input: DeleteProjectInput) -> FieldResult<ProjectOut> {
        let service_ctx = ctx.data::<ServiceContext<AwsDynamoDbProjectRepository<ES>>>().unwrap();

        let project_id = validate_project_id(&input.project_id)?;
        let executor_id = validate_user_id(&input.executor_id)?;

        let mut processor = service_ctx.project_command_processor.lock().await;
        processor
            .delete_project(project_id, executor_id)
            .await
            .map(|project_id| ProjectOut::new(project_id.to_string()))
            .map_err(error_handling)
    }

    async fn add_member(&self, ctx: &Context<'_>, input: AddMemberInput) -> FieldResult<ProjectOut> {
        let service_ctx = ctx.data::<ServiceContext<AwsDynamoDbProjectRepository<ES>>>().unwrap();

        let project_id = validate_project_id(&input.project_id)?;
        let user_id = validate_user_id(&input.user_id)?;
        let role = validate_member_role(&input.role)?;
        let executor_id = validate_user_id(&input.executor_id)?;

        let mut processor = service_ctx.project_command_processor.lock().await;
        processor
            .add_member(project_id, user_id, role, executor_id)
            .await
            .map(|project_id| ProjectOut::new(project_id.to_string()))
            .map_err(error_handling)
    }

    async fn remove_member(&self, ctx: &Context<'_>, input: RemoveMemberInput) -> FieldResult<ProjectOut> {
        let service_ctx = ctx.data::<ServiceContext<AwsDynamoDbProjectRepository<ES>>>().unwrap();

        let project_id = validate_project_id(&input.project_id)?;
        let user_id = validate_user_id(&input.user_id)?;
        let executor_id = validate_user_id(&input.executor_id)?;

        let mut processor = service_ctx.project_command_processor.lock().await;

        processor
            .remove_member(project_id, user_id, executor_id)
            .await
            .map(|project_id| ProjectOut::new(project_id.to_string()))
            .map_err(error_handling)
    }
}

fn error_handling_repository_error(error: &CommandProcessError, cause: &ProjectRepositoryError) -> Error {
    match cause {
        ProjectRepositoryError::StoreError(_, EventStoreWriteError::OptimisticLockError(_)) => {
            Error::new(error.to_string())
                .extend_with(|_, e| e.set("code", "409"))
                .extend_with(|_, e| e.set("cause", cause.to_string()))
        },
        ProjectRepositoryError::StoreError(_, _) => Error::new(error.to_string())
            .extend_with(|_, e| e.set("code", "500"))
            .extend_with(|_, e| e.set("cause", cause.to_string())),
        ProjectRepositoryError::FindByIdError(_, _) => Error::new(error.to_string())
            .extend_with(|_, e| e.set("code", "500"))
            .extend_with(|_, e| e.set("cause", cause.to_string())),
    }
}

fn error_handling(error: CommandProcessError) -> Error {
    match error {
        CommandProcessError::DomainLogicError(ref cause) => Error::new(error.to_string())
            .extend_with(|_, e| e.set("code", "422"))
            .extend_with(|_, e| e.set("cause", cause.to_string())),
        CommandProcessError::NotFoundError => Error::new(error.to_string()).extend_with(|_, e| e.set("code", "404")),
        CommandProcessError::RepositoryError(ref cause) => error_handling_repository_error(&error, cause),
    }
}

fn validate_project_id(value: &str) -> Result<ProjectId, Error> {
    ProjectId::from_str(value).map_err(|error| Error::new(error.to_string()).extend_with(|_, e| e.set("code", "400")))
}

fn validate_project_name(value: &str) -> Result<ProjectName, Error> {
    ProjectName::from_str(value).map_err(|error| Error::new(error.to_string()).extend_with(|_, e| e.set("code", "400")))
}

fn validate_member_role(value: &str) -> Result<MemberRole, Error> {
    MemberRole::from_str(value).map_err(|error| Error::new(error.to_string()).extend_with(|_, e| e.set("code", "400")))
}

fn validate_user_id(value: &str) -> Result<UserId, Error> {
    UserId::from_str(value).map_err(|error| Error::new(error.to_string()).extend_with(|_, e| e.set("code", "400")))
}
