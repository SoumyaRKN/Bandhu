use serde::Serialize;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug, Clone, Serialize)]
pub enum BackendError {
    Config(String),
    Io(String),
    Tool(String),
    Gate(String),
    Model(String),
    Queue(String),
    Http(String),
    Parse(String),
    Approval(String),
    NotFound(String),
    Timeout(String),
    Internal(String),
}

impl Display for BackendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(msg) => write!(f, "config error: {}", msg),
            Self::Io(msg) => write!(f, "io error: {}", msg),
            Self::Tool(msg) => write!(f, "tool error: {}", msg),
            Self::Gate(msg) => write!(f, "gate error: {}", msg),
            Self::Model(msg) => write!(f, "model error: {}", msg),
            Self::Queue(msg) => write!(f, "queue error: {}", msg),
            Self::Http(msg) => write!(f, "http error: {}", msg),
            Self::Parse(msg) => write!(f, "parse error: {}", msg),
            Self::Approval(msg) => write!(f, "approval error: {}", msg),
            Self::NotFound(msg) => write!(f, "not found: {}", msg),
            Self::Timeout(msg) => write!(f, "timeout: {}", msg),
            Self::Internal(msg) => write!(f, "internal error: {}", msg),
        }
    }
}

impl Error for BackendError {}

pub type BackendResult<T> = Result<T, BackendError>;

impl From<io::Error> for BackendError {
    fn from(err: io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<serde_json::Error> for BackendError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err.to_string())
    }
}

impl From<std::string::String> for BackendError {
    fn from(err: std::string::String) -> Self {
        Self::Internal(err)
    }
}

impl From<&str> for BackendError {
    fn from(err: &str) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<reqwest::Error> for BackendError {
    fn from(err: reqwest::Error) -> Self {
        Self::Http(err.to_string())
    }
}
