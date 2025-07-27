use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Schema, SchemaBuilder};
use event_store_adapter_rs::EventStoreForDynamoDB;
use tokio::sync::Mutex;

use command_domain::project::{Project, ProjectEvent, ProjectId};
use command_interface_adaptor_if::ProjectRepository;
use command_processor::project_command_processor::ProjectCommandProcessor;

use crate::gateways::project_repository::AwsDynamoDbProjectRepository;

pub mod inputs;
pub mod outputs;
pub mod resolvers;

pub struct ServiceContext<TR: ProjectRepository> {
    project_command_processor: Arc<Mutex<ProjectCommandProcessor<TR>>>,
}

impl<TR: ProjectRepository> ServiceContext<TR> {
    pub fn new(project_command_processor: ProjectCommandProcessor<TR>) -> Self {
        Self {
            project_command_processor: Arc::new(Mutex::new(project_command_processor)),
        }
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn health_check(&self) -> String {
        "OK".to_string()
    }
}

pub struct MutationRoot;

pub type ES = EventStoreForDynamoDB<ProjectId, Project, ProjectEvent>;

pub type ApiSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema_builder() -> SchemaBuilder<QueryRoot, MutationRoot, EmptySubscription> {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
}

pub fn create_schema(project_repository: AwsDynamoDbProjectRepository<ES>) -> ApiSchema {
    let processor = ProjectCommandProcessor::new(project_repository);
    let ctx = ServiceContext::new(processor);
    create_schema_builder().data(ctx).finish()
}
