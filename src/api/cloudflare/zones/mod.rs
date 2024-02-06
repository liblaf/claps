use crate::api::cloudflare::Cloudflare;

pub mod dns_records;

pub struct ClientZones {
    client: reqwest::Client,
    api: String,
    token: String,
    zone_id: String,
}

impl Cloudflare {
    pub fn zones(&self, zone_id: &str) -> ClientZones {
        ClientZones {
            client: self.client.to_owned(),
            api: self.api.to_string(),
            token: self.token.to_string(),
            zone_id: zone_id.to_string(),
        }
    }
}
