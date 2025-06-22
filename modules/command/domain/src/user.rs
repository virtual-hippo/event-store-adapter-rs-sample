mod user_error;
mod user_events;
pub mod user_id;
mod user_name;

use chrono::{DateTime, Utc};
use event_store_adapter_rs::types::Aggregate;
use serde::{Deserialize, Serialize};

use crate::email::Email;
use crate::user::user_error::UserError;
use crate::user::user_events::{UserEvent, UserEventCreatedBody, UserEventDeletedBody};
use crate::user::user_id::UserId;
use crate::user::user_name::UserName;

/// ユーザ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    deleted: bool,
    user_name: UserName,
    email: Email,
    seq_nr_counter: usize,
    version: usize,
    last_updated_at: DateTime<Utc>,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Aggregate for User {
    type ID = UserId;

    fn id(&self) -> &Self::ID {
        &self.id
    }

    fn seq_nr(&self) -> usize {
        self.seq_nr_counter
    }

    fn version(&self) -> usize {
        self.version
    }

    fn set_version(&mut self, version: usize) {
        self.version = version;
    }

    fn last_updated_at(&self) -> &DateTime<Utc> {
        &self.last_updated_at
    }
}

impl User {
    pub fn new(user_name: UserName, email: Email) -> (Self, UserEvent) {
        let id = UserId::default();
        Self::from(id, false, user_name, email, 0, 1)
    }

    pub fn delete(&mut self) -> Result<UserEvent, UserError> {
        if self.deleted {
            return Err(UserError::AlreadyDeletedError(self.id.clone()));
        }
        self.deleted = true;
        self.seq_nr_counter += 1;
        Ok(UserEvent::UserDeleted(UserEventDeletedBody::new(
            self.id.clone(),
            Utc::now(),
        )))
    }

    pub fn from(
        id: UserId,
        deleted: bool,
        user_name: UserName,
        email: Email,
        seq_nr_counter: usize,
        version: usize,
    ) -> (Self, UserEvent) {
        let now = Utc::now();
        (
            Self {
                id: id.clone(),
                deleted,
                user_name: user_name.clone(),
                email: email.clone(),
                seq_nr_counter,
                version,
                last_updated_at: now,
            },
            UserEvent::UserCreated(UserEventCreatedBody::new(id, user_name, email, now)),
        )
    }
}
