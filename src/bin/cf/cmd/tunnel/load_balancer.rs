use anyhow::Result;
use clap::Args;
use claps::{
    api::cloudflare::{accounts::cfd_tunnel::CfdTunnel, zones::dns_records::ClientDnsRecords},
    external::service::Service,
};

#[derive(Args)]
pub struct Cmd {
    #[arg(default_values = ["alist", "bt", "glances", "gpt", "jellyfin", "pdf"], ignore_case(true))]
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
