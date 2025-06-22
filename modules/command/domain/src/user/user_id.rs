use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::helper::{ParseError, id_generate};
use event_store_adapter_rs::types::AggregateId;
use serde::{Deserialize, Serialize};
use ulid_generator_rs::ULID;

const USER_PREFIX: &str = "User";

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UserId {
    value: ULID,
}

impl UserId {
    pub fn new() -> Self {
        let value = id_generate();
        Self { value }
    }

    pub fn from_ulid(value: ULID) -> Self {
        Self { value }
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl AggregateId for UserId {
    fn type_name(&self) -> String {
        USER_PREFIX.to_string()
    }

    fn value(&self) -> String {
        self.value.to_string()
    }
}

impl FromStr for UserId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = if s.starts_with(USER_PREFIX) {
            &s[(USER_PREFIX.len() + 1)..]
        } else {
            s
        };
        match ULID::from_str(ss) {
            Ok(value) => Ok(Self { value }),
            Err(err) => Err(ParseError::InvalidULID(err)),
        }
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.type_name(), self.value)
    }
}
