use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod block;
pub mod direct;
pub mod dns;
pub mod selector;
pub mod urltest;

use self::block::Block;
use self::direct::Direct;
use self::dns::Dns;
use self::selector::Selector;
use self::urltest::UrlTest;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum Outbound {
    Direct(Direct),
    Block(Block),
    Dns(Dns),
    Selector(Selector),
    UrlTest(UrlTest),
    #[serde(untagged)]
    Other(Other),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Other {
    pub tag: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(flatten)]
    pub other: Value,
}
