use anyhow::Result;
use clap::Args;
use claps::{
    api::cloudflare::{
        accounts::cfd_tunnel::{
            configurations::{Config, Ingress},
            CfdTunnel, ClientCfdTunnel,
        },
        zones::dns_records::ClientDnsRecords,
    },
    external::service::Service,
};

use super::TunnelZoneArgs;

#[derive(Args)]
pub struct Cmd {
    #[arg(default_values = ["alist", "bt", "glances", "gpt", "jellyfin", "pdf"], ignore_case(true))]
    services: Vec<Service>,
    #[command(flatten)]
    args: TunnelZoneArgs,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let services = self.services;
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
        let client = self.args.accounts().await?;
        let client = client.cfd_tunnel();
        let tunnel = client
            .get(
                self.args
                    .name
                    .as_deref()
                    .or(Some(whoami::devicename().as_str())),
            )
            .await?;
        let tunnel = tunnel.first().unwrap();
        let client_tunnel = self.args.accounts().await?.cfd_tunnel();
        let client_dns = self.args.zones().await?.dns_records();
        tokio::try_join!(
            update_tunnel(&client_tunnel, tunnel, services.as_slice()),
            update_dns(&client_dns, tunnel, services.as_slice())
        )?;
        Ok(())
    }
}

async fn update_tunnel(
    client: &ClientCfdTunnel,
    tunnel: &CfdTunnel,
    services: &[Service],
) -> Result<()> {
    let config = Config {
        ingress: services
            .iter()
            .map(Service::ingress_main)
            .chain(
                services
                    .iter()
                    .map(|s| s.ingress(Some(tunnel.name.as_str()))),
            )
            .chain([Ingress {
                hostname: None,
                service: "http_status:404".to_string(),
            }])
            .collect(),
    };
    let client = client.configurations(tunnel.id.as_str());
    println!("{}", config);
    client.put(&config).await
}

async fn update_dns(
    client: &ClientDnsRecords,
    tunnel: &CfdTunnel,
    services: &[Service],
) -> Result<()> {
    let records = client.get(None).await?;
    let records = records
        .into_iter()
        .filter(|r| {
            r.name
                .ends_with(format!("-{}.liblaf.me", tunnel.name.to_lowercase()).as_str())
        })
        .collect::<Vec<_>>();
    let names = services
        .iter()
        .map(|s| s.hostname(Some(tunnel.name.as_str())))
        .collect::<Vec<_>>();
    futures::future::try_join_all(records.iter().filter_map(|r| {
        if names.contains(&r.name) {
            None
        } else {
            Some(client.delete(r.id.as_str(), Some(r)))
        }
    }))
    .await?;
    futures::future::try_join_all(services.iter().map(|s| async {
        let name = s.hostname(Some(tunnel.name.as_str()));
        let content = format!("{}.cfargotunnel.com", tunnel.id);
        let records = records
            .iter()
            .filter(|r| r.name == name)
            .collect::<Vec<_>>();
        let mut exists = false;
        for record in records {
            if record.content == content {
                tracing::info!("DNS Record Exists: {}", record);
                exists = true;
            } else {
                client.delete(record.id.as_str(), Some(record)).await?;
            }
        }
        if !exists {
            client
                .post(content, name, Some(true), "CNAME".to_string(), None)
                .await?;
        }
        Ok::<_, anyhow::Error>(())
    }))
    .await?;
    Ok(())
}
