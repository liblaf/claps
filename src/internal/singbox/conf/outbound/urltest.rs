use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UrlTest {
    pub tag: String,
    pub outbounds: Vec<String>,
    pub url: Option<String>,
}
