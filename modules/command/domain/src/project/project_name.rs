use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectName(String);

#[derive(Error, Debug, Clone)]
pub enum ProjectNameError {
    #[error("the project is empty")]
    Empty,
    #[error("the project is too long")]
    TooLong,
}

impl ProjectName {
    pub fn new(name: &str) -> Result<Self, ProjectNameError> {
        if name.is_empty() {
            Err(ProjectNameError::Empty)
        } else if name.len() > 100 {
            Err(ProjectNameError::TooLong)
        } else {
            Ok(Self(name.to_string()))
        }
    }
}

impl FromStr for ProjectName {
    type Err = ProjectNameError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Display for ProjectName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
