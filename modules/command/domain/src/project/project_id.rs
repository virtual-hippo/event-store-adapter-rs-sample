use crate::helper::{ParseError, id_generate};
use event_store_adapter_rs::types::AggregateId;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use ulid_generator_rs::ULID;

#[derive(Debug, Clone, Eq, Hash, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct ProjectId {
    value: ULID,
}

const PROJECT_PREFIX: &str = "Project";

impl ProjectId {
    pub fn new() -> Self {
        let value = id_generate();
        Self { value }
    }
}

impl AggregateId for ProjectId {
    fn type_name(&self) -> String {
        PROJECT_PREFIX.to_string()
    }

    fn value(&self) -> String {
        self.value.to_string()
    }
}

impl Default for ProjectId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for ProjectId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.type_name(), self.value)
    }
}

impl From<ULID> for ProjectId {
    fn from(value: ULID) -> Self {
        Self { value }
    }
}

impl FromStr for ProjectId {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = if s.starts_with(PROJECT_PREFIX) {
            &s[(PROJECT_PREFIX.len() + 1)..]
        } else {
            s
        };
        match ULID::from_str(ss) {
            Ok(value) => Ok(Self { value }),
            Err(err) => Err(ParseError::InvalidULID(err)),
        }
    }
}
