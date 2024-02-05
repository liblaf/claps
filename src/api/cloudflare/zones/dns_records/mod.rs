use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::api::cloudflare::zones::ClientZones;

mod delete;
mod get;
mod post;

pub struct ClientDnsRecords {
    client: reqwest::Client,
    api: String,
    token: String,
    zone_id: String,
}

impl ClientZones {
    pub fn dns_records(&self) -> ClientDnsRecords {
        ClientDnsRecords {
            client: self.client.to_owned(),
            api: self.api.to_string(),
            token: self.token.to_string(),
            zone_id: self.zone_id.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DnsRecord {
    pub content: String,
    pub name: String,
    pub proxied: bool,
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
    pub ttl: u64,
}

impl Display for DnsRecord {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}s {}",
            self.name, self.type_, self.ttl, self.content
        )
    }
}
