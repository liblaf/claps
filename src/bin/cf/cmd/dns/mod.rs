use anyhow::Result;
use clap::{Args, Subcommand};
use claps::api::cloudflare::zones::Zones;

use super::CloudflareArgs;

mod delete;
mod install;
mod list;
mod update;

#[derive(Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[command(flatten)]
    args: ZonesArgs,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        match self.cmd {
            SubCmd::Delete(cmd) => cmd.run().await,
            SubCmd::Install(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run().await,
            SubCmd::Update(cmd) => cmd.run().await,
        }
    }
}

#[derive(Subcommand)]
enum SubCmd {
    Delete(delete::Cmd),
    Install(install::Cmd),
    List(list::Cmd),
    Update(update::Cmd),
}

#[derive(Clone, Args)]
struct ZonesArgs {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
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

#[derive(Clone, Args)]
pub struct ZonesArgsFromGlobal {
    #[arg(from_global)]
    pub api: String,
    #[arg(from_global)]
    pub token: Option<String>,
    #[arg(from_global)]
    pub zone: String,
    #[arg(from_global)]
    pub name: Option<String>,
}

impl ZonesArgsFromGlobal {
    fn args(&self) -> CloudflareArgs {
        CloudflareArgs {
            api: self.api.to_string(),
            token: self.token.to_owned(),
        }
    }

    async fn name(&self) -> Result<String> {
        if let Some(name) = self.name.as_deref() {
            return Ok(name.to_string());
        }
        Ok(format!("{}.ddns.liblaf.me", whoami::hostname()))
    }

    pub async fn zones(&self) -> Result<Zones> {
        let cloudflare = self.args().cloudflare().await?;
        Ok(cloudflare.zones(self.zone.as_str()))
    }
}
