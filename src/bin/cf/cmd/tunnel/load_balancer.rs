use anyhow::Result;
use clap::Args;
use claps::{
    api::cloudflare::{accounts::cfd_tunnel::CfdTunnel, zones::dns_records::ClientDnsRecords},
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
        let client = self.args.accounts().await?;
        let _tunnels = client.cfd_tunnel().get(None).await?;
        todo!()
    }
}

async fn update_service(
    _client: &ClientDnsRecords,
    _service: Service,
    tunnels: &[CfdTunnel],
) -> Result<()> {
    for _tunnel in tunnels {}
    todo!()
}
