use crate::project::{ProjectId, ProjectName};
use crate::user::UserId;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("The project is deleted: {0:?}")]
    AlreadyDeletedError(ProjectId),

    #[error("The {0} is not an administrator of the project: {1:?}")]
    NotAdministratorError(String, UserId),

    #[error("The {0} is not a member of the project: {1:?}")]
    NotMemberError(String, UserId),

    #[error("The {0} is already a member of the project: {1:?}")]
    AlreadyMemberError(String, UserId),

    #[error("Both {0} and {1} are not mismatched")]
    MismatchedUserError(String, String),

    #[error("The project name is already exists: {0:?}, {1:?}")]
    AlreadyExistsNameError(ProjectId, ProjectName),
}
