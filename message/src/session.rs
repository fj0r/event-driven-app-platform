use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref};
use serde_json::Value;

pub type SessionId = String;
pub type SessionCount = u128;

#[derive(Clone, Debug, Deserialize, Serialize, Default, PartialEq, Eq, Hash)]
pub struct Session(pub SessionId);

impl Deref for Session {
    type Target = SessionId;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Session {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<Session> for Value {
    fn from(value: Session) -> Self {
        value.0.into()
    }
}

impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<SessionCount> for Session {
    fn from(value: SessionCount) -> Self {
        Self(value.to_string())
    }
}
