use serde::{Deserialize, Serialize};

pub mod mixed;

use mixed::Mixed;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Inbound {
    Mixed(Mixed),
}
