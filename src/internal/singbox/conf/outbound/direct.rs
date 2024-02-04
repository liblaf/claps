use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Direct {
    pub tag: String,
}

impl Default for Direct {
    fn default() -> Self {
        Self {
            tag: "DIRECT".to_string(),
        }
    }
}
