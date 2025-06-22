use std::fmt::{Display, Formatter};
use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Email(String);

#[derive(Error, Debug, Clone)]
pub enum EmailError {
    #[error("the email is empty")]
    Empty,
    #[error("the email is too long")]
    TooLong,
    #[error("the email is invalid format")]
    InvalidFormat,
}

impl Email {
    pub fn new(email: &str) -> Result<Self, EmailError> {
        if email.is_empty() {
            return Err(EmailError::Empty);
        }

        if email.len() > 100 {
            return Err(EmailError::TooLong);
        }

        let re = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
            .map_err(|_| EmailError::InvalidFormat)?;
        if !re.is_match(email) {
            return Err(EmailError::InvalidFormat);
        }

        Ok(Self(email.to_string()))
    }
}

impl FromStr for Email {
    type Err = EmailError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
