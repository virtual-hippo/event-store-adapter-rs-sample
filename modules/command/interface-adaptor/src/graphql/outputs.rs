use async_graphql::SimpleObject;

#[derive(Debug, Clone, SimpleObject)]
pub struct ProjectOut {
    project_id: String,
}

impl ProjectOut {
    pub fn new(project_id: String) -> Self {
        Self { project_id }
    }
}
