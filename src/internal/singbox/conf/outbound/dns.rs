use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dns {
    pub tag: String,
}

impl Default for Dns {
    fn default() -> Self {
        Self {
            tag: "DNS".to_string(),
        }
    }
}
