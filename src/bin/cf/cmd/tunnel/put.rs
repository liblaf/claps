use anyhow::Result;
use clap::Args;
use claps::{
    api::cloudflare::accounts::cfd_tunnel::configurations::{Config, Ingress},
    external::service::Service,
};

use super::AccountsArgsFromGlobal;

#[derive(Args)]
pub struct Cmd {
    #[command(flatten)]
    args: AccountsArgsFromGlobal,
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
                Ok(()) => Some(s),
                Err(_) => None,
            });
        let config = Config {
            ingress: services
                .map(Ingress::from)
                .chain([Ingress {
                    hostname: None,
                    service: "http_status:404".to_string(),
                }])
                .collect(),
        };
        println!("{}", config);
        let client = self.args.accounts().await?;
        let client = client.cfd_tunnel();
        let cfd_tunnel = client.get(Some(whoami::devicename().as_str())).await?;
        let cfd_tunnel = cfd_tunnel.first().unwrap();
        let client = client.configurations(cfd_tunnel.id.as_str());
        client.put(&config).await?;
        Ok(())
    }
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
