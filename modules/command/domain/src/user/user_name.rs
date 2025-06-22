use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserName(String);

#[derive(Error, Debug, Clone)]
pub enum UserNameError {
    #[error("the user name is empty")]
    Empty,
    #[error("the user name is too long")]
    TooLong,
}

impl UserName {
    pub fn new(name: &str) -> Result<Self, UserNameError> {
        if name.is_empty() {
            Err(UserNameError::Empty)
        } else if name.len() > 100 {
            Err(UserNameError::TooLong)
        } else {
            Ok(Self(name.to_string()))
        }
    }
}

impl FromStr for UserName {
    type Err = UserNameError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Display for UserName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
