use anyhow::Result;
use clap::Args;
use claps::{
    api::cloudflare::accounts::cfd_tunnel::configurations::{Config, Ingress},
    external::service::Service,
};

use crate::cmd::dns::ZonesArgsFromGlobal;

use super::AccountsArgsFromGlobal;

#[derive(Args)]
pub struct Cmd {
    #[command(flatten)]
    zone: ZonesArgs,
    #[command(flatten)]
    accounts: AccountsArgsFromGlobal,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let services = SERVICES;
        let connectivity = services.iter().map(|s| s.test());
        let connectivity = futures::future::join_all(connectivity).await;
        let services = services
            .iter()
            .enumerate()
            .filter_map(|(i, s)| match connectivity[i] {
                Ok(()) => Some(s.to_owned()),
                Err(_) => None,
            })
            .collect::<Vec<_>>();
        let client = self.accounts.accounts().await?;
        let client = client.cfd_tunnel();
        let tunnel = client.get(Some(whoami::devicename().as_str())).await?;
        let tunnel = tunnel.first().unwrap();
        let zones_args = ZonesArgsFromGlobal {
            api: self.accounts.api.to_string(),
            token: self.accounts.token.to_owned(),
            zone: self.zone.zone.to_string(),
            name: self.zone.name.to_owned(),
        };
        tokio::try_join!(
            update_tunnel(&self.accounts, tunnel.id.as_str(), services.as_slice()),
            update_dns(&zones_args, tunnel.id.as_str(), services.as_slice())
        )?;
        Ok(())
    }
}

#[derive(Args)]
struct ZonesArgs {
    #[arg(
        short,
        long,
        default_value = "919b04037636d3b4bbc0af135eaccdfa",
        global = true
    )]
    zone: String,
    #[arg(short, long, global = true)]
    name: Option<String>,
}

const SERVICES: &[Service] = &[
    Service::AList,
    Service::BT,
    // Service::DNS,
    Service::Glances,
    Service::GPT,
    Service::Jellyfin,
    Service::PDF,
];

async fn update_tunnel(
    args: &AccountsArgsFromGlobal,
    tunnel_id: &str,
    services: &[Service],
) -> Result<()> {
    let config = Config {
        ingress: services
            .iter()
            .map(Service::ingress_main)
            .chain(services.iter().map(Ingress::from))
            .chain([Ingress {
                hostname: None,
                service: "http_status:404".to_string(),
            }])
            .collect(),
    };
    let client = args.accounts().await?;
    let client = client.cfd_tunnel();
    let client = client.configurations(tunnel_id);
    println!("{}", config);
    client.put(&config).await
}

async fn update_dns(
    args: &ZonesArgsFromGlobal,
    tunnel_id: &str,
    services: &[Service],
) -> Result<()> {
    let client = args.zones().await?;
    let client = client.dns_records();
    futures::future::try_join_all(services.iter().map(|s| async {
        let name = s.hostname();
        let content = format!("{}.cfargotunnel.com", tunnel_id);
        let records = client.get(Some(name.as_str())).await?;
        if let Some(record) = records.first() {
            if record.content == content {
                tracing::info!("DNS Record Exists: {}", record);
                return Ok(());
            }
            client.delete(record.id.as_str(), Some(record)).await?;
        }
        client
            .post(content, name, Some(true), "CNAME".to_string(), None)
            .await
    }))
    .await?;
    Ok(())
}
