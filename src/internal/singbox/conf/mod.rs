use std::vec;

use serde::{Deserialize, Serialize};

pub mod dns;
pub mod inbound;
pub mod log;
pub mod outbound;
pub mod route;
pub mod shared;

use self::dns::Dns;
use self::inbound::mixed::Mixed;
use self::inbound::Inbound;
use self::log::Log;
use self::outbound::block::Block;
use self::outbound::direct::Direct;
use self::outbound::selector::Selector;
use self::outbound::Outbound;
use self::route::Route;
use self::shared::listen::Listen;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub log: Option<Log>,
    pub dns: Option<Dns>,
    pub inbounds: Option<Vec<Inbound>>,
    pub outbounds: Option<Vec<Outbound>>,
    pub route: Option<Route>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            log: Some(Log::default()),
            dns: Some(Dns::default()),
            inbounds: Some(vec![Inbound::Mixed(Mixed {
                tag: "in-mixed".to_string(),
                listen: Listen {
                    listen: "127.0.0.1".to_string(),
                    listen_port: Some(2080),
                },
                ..Default::default()
            })]),
            outbounds: Some(vec![
                Outbound::Selector(Selector {
                    tag: "PROXY".to_string(),
                    ..Default::default()
                }),
                Outbound::Dns(self::outbound::dns::Dns::default()),
                Outbound::Direct(Direct::default()),
                Outbound::Block(Block::default()),
            ]),
            route: Some(Route::default()),
        }
    }
}
