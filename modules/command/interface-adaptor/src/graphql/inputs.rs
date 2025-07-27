use async_graphql::InputObject;

#[derive(Debug, Clone, InputObject)]
pub struct CreateProjectInput {
    pub name: String,
    pub executor_id: String,
}

#[derive(Debug, Clone, InputObject)]
pub struct DeleteProjectInput {
    pub project_id: String,
    pub executor_id: String,
}

#[derive(Debug, Clone, InputObject)]
pub struct AddMemberInput {
    pub project_id: String,
    pub user_id: String,
    pub role: String,
    pub executor_id: String,
}

#[derive(Debug, Clone, InputObject)]
pub struct RemoveMemberInput {
    pub project_id: String,
    pub user_id: String,
    pub executor_id: String,
}
