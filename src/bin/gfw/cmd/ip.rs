use std::fmt::Display;
use std::time::Duration;

use anyhow::Result;
use clap::Args;
use colored::Colorize;
use reqwest::{Client, ClientBuilder};
use serde::Deserialize;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::Format;
use tabled::settings::{Modify, Style};

use claps::common::cmd::Run;
use claps::common::log::LogResult;

#[derive(Debug, Args)]
pub struct Cmd {
    #[arg(short, long, value_parser = parse_duration, default_value = "3")]
    timeout: Duration,
}

fn parse_duration(arg: &str) -> Result<Duration> {
    let seconds = arg.parse().log()?;
    Ok(Duration::from_secs(seconds))
}

async fn get_ip(client: &Client, url: &str) -> Result<String> {
    let request = client.get(url).build().log()?;
    Ok(client
        .execute(request)
        .await?
        .text()
        .await
        .log()?
        .trim()
        .to_string())
}

async fn get_geo_ip(client: &Client) -> Result<GeoIP> {
    let request = client.get("https://api.ip.sb/geoip").build().log()?;
    client.execute(request).await?.json().await.log()
}

#[derive(Debug, Deserialize)]
struct GeoIP {
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
}

impl Display for GeoIP {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut builder = Builder::new();
        builder.push_record(["IP", &self.ip]);
        builder.push_record(["Country Code", &self.country_code]);
        builder.push_record(["Country", &self.country]);
        if let Some(region_code) = &self.region_code {
            builder.push_record(["Region Code", region_code]);
        }
        if let Some(region) = &self.region {
            builder.push_record(["Region", region]);
        }
        if let Some(city) = &self.city {
            builder.push_record(["City", city]);
        }
        if let Some(postal_code) = &self.postal_code {
            builder.push_record(["Postal Code", postal_code]);
        }
        builder.push_record(["Continent Code", &self.continent_code]);
        builder.push_record(["Latitude", &self.latitude.to_string()]);
        builder.push_record(["Longitude", &self.longitude.to_string()]);
        builder.push_record(["Organization", &self.organization]);
        builder.push_record(["Timezone", &self.timezone]);
        let mut table = builder.build();
        table
            .with(Style::blank())
            .with(
                Modify::new(Columns::first())
                    .with(Format::content(|s| s.bold().yellow().to_string())),
            )
            .with(
                Modify::new(Columns::single(1))
                    .with(Format::content(|s| s.bold().magenta().to_string())),
            );
        write!(f, "{}", table)
    }
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        let client = ClientBuilder::new().timeout(self.timeout).build().log()?;
        println!("{}", "-".repeat(57));
        for v in [4, 6] {
            match get_ip(&client, &format!("https://api-ipv{}.ip.sb/ip", v)).await {
                Ok(ip) => println!(
                    "{}: {}",
                    format!("IPv{}", v).bold().yellow(),
                    ip.to_string().bold().magenta()
                ),
                Err(_) => println!(
                    "{}: {}",
                    format!("IPv{}", v).bold().yellow(),
                    "-".bold().magenta()
                ),
            }
            println!("{}", "-".repeat(57));
        }
        match get_geo_ip(&client).await {
            Ok(geo_ip) => println!("{}", geo_ip),
            Err(_) => println!("{}: {}", "GeoIP".bold().yellow(), "-".bold().magenta()),
        }
        println!("{}", "-".repeat(57));
        Ok(())
    }
}
