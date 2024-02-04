use serde::{Deserialize, Serialize};

use crate::internal::singbox::conf::shared::listen::Listen;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Mixed {
    pub tag: String,
    #[serde(flatten)]
    pub listen: Listen,
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}
