use anyhow::Result;
use clap::Args;
use claps::{
    api::cloudflare::{
        accounts::cfd_tunnel::CfdTunnel,
        zones::dns_records::{ClientDnsRecords, DnsRecord},
    },
    external::service::Service,
};

const DEFAULT_SERVICES: &[&str] = &["alist", "gpt", "jellyfin", "pdf"];

#[derive(Args)]
pub struct Cmd {
    #[arg(default_values(DEFAULT_SERVICES), ignore_case(true))]
    services: Vec<Service>,
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    account: String,
    #[arg(from_global)]
    zone: String,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let client = crate::helper::client::accounts(
            self.api.as_str(),
            self.token.as_deref(),
            self.account.as_str(),
        )
        .await?;
        let mut tunnels = client.cfd_tunnel().get(None).await?;
        tunnels.sort_unstable_by(|a, b| a.status.cmp(&b.status));
        tracing::debug!("{:?}", tunnels);
        let client_dns = crate::helper::client::zones(
            self.api.as_str(),
            self.token.as_deref(),
            self.zone.as_str(),
        )
        .await?
        .dns_records();
        let records = client_dns.get(None).await?;
        futures::future::try_join_all(
            self.services
                .iter()
                .map(|s| update_service(&client_dns, s, tunnels.as_slice(), records.as_slice())),
        )
        .await?;
        Ok(())
    }
}

async fn update_service(
    client: &ClientDnsRecords,
    service: &Service,
    tunnels: &[CfdTunnel],
    records: &[DnsRecord],
) -> Result<()> {
    let hostname_balancer = service.hostname_balancer();
    let record_balancer = records.iter().find(|r| r.name == hostname_balancer);
    for tunnel in tunnels {
        let hostname_server = service.hostname_server(Some(tunnel.name.as_str()));
        let record_server = records.iter().find(|r| r.name == hostname_server);
        if record_server.is_none() {
            continue;
        }
        let server = record_server.unwrap();
        if let Some(balancer) = record_balancer {
            if balancer.content == server.content {
                tracing::info!("DNS Record Exists: {}", balancer);
                tracing::info!("{} -> {}", hostname_balancer, tunnel.name);
                return Ok(());
            } else {
                client.delete(balancer.id.as_str(), Some(balancer)).await?;
            }
        }
        client
            .post(
                server.content.to_string(),
                hostname_balancer.to_string(),
                Some(true),
                "CNAME".to_string(),
                None,
            )
            .await?;
        tracing::info!("{} -> {}", hostname_balancer, tunnel.name);
        return Ok(());
    }
    tracing::warn!("No Tunnel Found for Service: {}", service);
    Ok(())
}
