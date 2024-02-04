use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Log {
    pub disabled: Option<bool>,
    pub level: Option<Level>,
    pub timestamp: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Panic,
}

impl Default for Log {
    fn default() -> Self {
        Log {
            disabled: Some(false),
            level: Some(Level::Info),
            timestamp: Some(true),
        }
    }
}
