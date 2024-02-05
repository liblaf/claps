use anyhow::Result;
use clap::{Args, Subcommand};
use claps::api::cloudflare::{accounts::ClientAccounts, zones::ClientZones};

use super::CloudflareArgs;

mod get;
mod load_balancer;
mod put;

#[derive(Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[command(flatten)]
    args: TunnelArgs,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        match self.cmd {
            SubCmd::Get(cmd) => cmd.run().await,
            SubCmd::LoadBalancer(cmd) => cmd.run().await,
            SubCmd::Put(cmd) => cmd.run().await,
        }
    }
}

#[derive(Subcommand)]
enum SubCmd {
    Get(get::Cmd),
    LoadBalancer(load_balancer::Cmd),
    Put(put::Cmd),
}

#[derive(Args)]
struct TunnelArgs {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(
        long,
        default_value = "7ad40aa54c5d9453abe45eeb3c6643de",
        global = true
    )]
    account: String,
    #[arg(short, long, global(true))]
    name: Option<String>,
}

#[derive(Args)]
struct TunnelArgsFromGlobal {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    account: String,
    #[arg(from_global)]
    name: Option<String>,
}

impl TunnelArgsFromGlobal {
    fn args(&self) -> CloudflareArgs {
        CloudflareArgs {
            api: self.api.to_string(),
            token: self.token.to_owned(),
        }
    }

    async fn accounts(&self) -> Result<ClientAccounts> {
        let cloudflare = self.args().cloudflare().await?;
        Ok(cloudflare.accounts(self.account.as_str()))
    }
}

#[derive(Args)]
struct TunnelZoneArgs {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    account: String,
    #[arg(from_global)]
    name: Option<String>,
    #[arg(
        short,
        long,
        default_value = "919b04037636d3b4bbc0af135eaccdfa",
        global = true
    )]
    zone: String,
}

impl TunnelZoneArgs {
    async fn accounts(&self) -> Result<ClientAccounts> {
        let args = TunnelArgsFromGlobal {
            api: self.api.to_string(),
            token: self.token.to_owned(),
            account: self.account.to_string(),
            name: self.name.to_owned(),
        };
        args.accounts().await
    }

    async fn zones(&self) -> Result<ClientZones> {
        let args = CloudflareArgs {
            api: self.api.to_string(),
            token: self.token.to_owned(),
        };
        let client = args.cloudflare().await?;
        let client = client.zones(self.zone.as_str());
        Ok(client)
    }
}
