use crate::user::UserId;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum UserError {
    #[error("The user is deleted: {0:?}")]
    AlreadyDeletedError(UserId),
}
