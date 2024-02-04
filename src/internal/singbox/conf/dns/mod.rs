use serde::{Deserialize, Serialize};

mod rule;
mod server;

use self::rule::Rule;
use self::server::Server;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dns {
    pub servers: Option<Vec<Server>>,
    pub rules: Option<Vec<Rule>>,
    #[serde(rename = "final")]
    pub final_: Option<String>,
}

impl Default for Dns {
    fn default() -> Self {
        Self {
            servers: Some(vec![
                Server {
                    tag: "dns-!cn".to_string(),
                    address: "https://cloudflare-dns.com/dns-query".to_string(),
                    address_resolver: Some("dns-cn".to_string()),
                    detour: Some("PROXY".to_string()),
                },
                Server {
                    tag: "dns-cn".to_string(),
                    address: "https://dns.tuna.tsinghua.edu.cn:8443/dns-query".to_string(),
                    address_resolver: Some("dns-local".to_string()),
                    detour: Some("DIRECT".to_string()),
                },
                Server {
                    tag: "dns-local".to_string(),
                    address: "local".to_string(),
                    detour: Some("DIRECT".to_string()),
                    ..Default::default()
                },
                Server {
                    tag: "dns-block".to_string(),
                    address: "rcode://success".to_string(),
                    ..Default::default()
                },
            ]),
            rules: Some(vec![
                Rule {
                    outbound: Some(vec!["any".to_string()]),
                    server: "dns-cn".to_string(),
                    ..Default::default()
                },
                Rule {
                    rule_set: Some(vec!["geosite:category-ads-all".to_string()]),
                    server: "dns-block".to_string(),
                    disable_cache: Some(true),
                    ..Default::default()
                },
                Rule {
                    clash_mode: Some("direct".to_string()),
                    server: "dns-local".to_string(),
                    ..Default::default()
                },
                Rule {
                    clash_mode: Some("global".to_string()),
                    server: "dns-!cn".to_string(),
                    ..Default::default()
                },
                Rule {
                    rule_set: Some(vec!["geosite:cn".to_string()]),
                    server: "dns-cn".to_string(),
                    ..Default::default()
                },
            ]),
            final_: Some("dns-!cn".to_string()),
        }
    }
}
