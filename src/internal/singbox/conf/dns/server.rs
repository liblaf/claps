use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Server {
    pub tag: String,
    pub address: String,
    pub address_resolver: Option<String>,
    pub detour: Option<String>,
}
