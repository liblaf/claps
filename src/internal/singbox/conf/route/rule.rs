use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Rule {
    pub auth_user: Option<Vec<String>>,
    pub protocol: Option<Vec<String>>,
    pub ip_is_private: Option<bool>,
    pub clash_mode: Option<String>,
    pub rule_set: Option<Vec<String>>,
    pub outbound: String,
}
