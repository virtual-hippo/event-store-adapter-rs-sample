use crate::email::Email;
use crate::user::user_id::UserId;
use crate::user::user_name::UserName;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UserEvent {
    /// ユーザアカウントが作成された
    UserCreated(UserEventCreatedBody),
    /// ユーザアカウントが削除された
    UserDeleted(UserEventDeletedBody),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEventCreatedBody {
    pub aggregate_id: UserId,
    pub user_name: UserName,
    pub email: Email,
    pub occurred_at: DateTime<Utc>,
}

impl UserEventCreatedBody {
    pub fn new(
        aggregate_id: UserId,
        user_name: UserName,
        email: Email,
        occurred_at: DateTime<Utc>,
    ) -> UserEventCreatedBody {
        Self {
            aggregate_id,
            user_name,
            email,
            occurred_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEventDeletedBody {
    pub aggregate_id: UserId,
    pub occurred_at: DateTime<Utc>,
}

impl UserEventDeletedBody {
    pub fn new(aggregate_id: UserId, occurred_at: DateTime<Utc>) -> UserEventDeletedBody {
        Self { aggregate_id, occurred_at }
    }
}
