use anyhow::Result;
use clap::Args;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::common::cmd::{Run, STYLES};
use claps::common::log::LogInit;

mod delete;
mod list;
mod update;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, styles = STYLES)]
pub(super) struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,

    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd<Cmd>),
    Delete(delete::Cmd),
    List(list::Cmd),
    Update(update::Cmd),
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            SubCmd::Complete(cmd) => cmd.run().await,
            SubCmd::Delete(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run().await,
            SubCmd::Update(cmd) => cmd.run().await,
        }
    }
}

#[derive(Debug, Args)]
struct CommonArgs {
    #[clap(short, long, env)]
    name: Option<String>,
    #[clap(short, long, env)]
    token: Option<String>,
    #[clap(short, long, env, default_value = "919b04037636d3b4bbc0af135eaccdfa")]
    zone: String,
}

impl CommonArgs {
    pub fn name(&self) -> Result<String> {
        if let Some(name) = &self.name {
            return Ok(name.to_string());
        }
        Ok(whoami::hostname() + ".ddns.liblaf.me")
    }

    pub fn token(&self) -> Result<String> {
        if let Some(token) = &self.token {
            return Ok(token.to_string());
        }
        if let Ok(token) = claps::external::bitwarden::get_notes("CLOUDFLARE_TOKEN") {
            return Ok(token);
        }
        claps::bail!("token was not provided");
    }

    pub fn zone(&self) -> Result<String> {
        Ok(self.zone.to_string())
    }
}
