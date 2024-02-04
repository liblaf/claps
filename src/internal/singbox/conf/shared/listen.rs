use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Listen {
    pub listen: String,
    pub listen_port: Option<u16>,
}
