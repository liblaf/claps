use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub tag: String,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            tag: "BLOCK".to_string(),
        }
    }
}
