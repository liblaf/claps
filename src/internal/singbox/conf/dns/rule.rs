use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Rule {
    pub auth_user: Option<Vec<String>>,
    pub clash_mode: Option<String>,
    pub rule_set: Option<Vec<String>>,
    pub outbound: Option<Vec<String>>,
    pub server: String,
    pub disable_cache: Option<bool>,
}
