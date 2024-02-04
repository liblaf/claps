use std::fmt::{Display, Formatter};
use std::net::IpAddr;

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::{Color, Style};
use tabled::Table;

use crate::common::log::{LogJson, LogResult};

pub enum IPVersion {
    V4,
    V6,
}

fn api(v: Option<IPVersion>) -> &'static str {
    match v {
        Some(IPVersion::V4) => "https://api-ipv4.ip.sb",
        Some(IPVersion::V6) => "https://api-ipv6.ip.sb",
        None => "https://api.ip.sb",
    }
}

pub async fn ip(v: Option<IPVersion>) -> Result<IpAddr> {
    let client = Client::new();
    let request = client.get(format!("{}/ip", api(v)));
    let response = request.send().await.log()?;
    let response = response.error_for_status().log()?;
    let text = response.text().await.log()?;
    text.trim().parse().log()
}

pub async fn geoip(addr: Option<IpAddr>, v: Option<IPVersion>) -> Result<GeoIP> {
    let client = Client::new();
    let request = if let Some(addr) = addr {
        client.get(format!("{}/geoip/{}", api(v), addr))
    } else {
        client.get(format!("{}/geoip", api(v)))
    };
    let response = request.send().await.log()?;
    let response = response.error_for_status().log()?;
    Ok(response.json_log::<GeoIP>().await?)
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeoIP {
    ip: String,
    country_code: String,
    country: String,
    region_code: Option<String>,
    region: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    continent_code: String,
    latitude: f64,
    longitude: f64,
    organization: String,
    timezone: String,
    #[serde(flatten)]
    extra: Option<Value>,
}

impl GeoIP {
    pub fn table(&self) -> Table {
        let mut table = Builder::new();
        table.push_record(["IP", self.ip.as_str()]);
        table.push_record(["Country", self.country.as_str()]);
        if let Some(resion) = self.region.as_deref() {
            table.push_record(["Region", resion]);
        }
        if let Some(city) = self.city.as_deref() {
            table.push_record(["City", city]);
        }
        table.push_record(["Latitude", self.latitude.to_string().as_str()]);
        table.push_record(["Longitude", self.longitude.to_string().as_str()]);
        table.push_record(["Organization", self.organization.as_str()]);
        table.push_record(["Timezone", self.timezone.as_str()]);
        table.build()
    }
}
