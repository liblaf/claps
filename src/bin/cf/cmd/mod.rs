use anyhow::Result;
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::api::cloudflare::Cloudflare;
use claps::common::log::LogInit;

mod dns;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, about, styles = claps::common::cmd::STYLES)]
pub struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
    #[command(flatten)]
    args: CloudflareArgs,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd),
    Dns(dns::Cmd),
}

#[derive(Args)]
struct CloudflareArgs {
    #[arg(short, long, default_value = claps::api::cloudflare::API, global = true)]
    api: String,
    #[arg(short, long, global = true)]
    token: Option<String>,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            SubCmd::Complete(cmd) => cmd.run(Cmd::command()),
            SubCmd::Dns(cmd) => cmd.run().await,
        }
    }
}

impl CloudflareArgs {
    async fn token(&self) -> Result<String> {
        if let Some(token) = self.token.as_deref() {
            return Ok(token.to_string());
        }
        claps::external::bw::get::notes("CLOUDFLARE_TOKEN").await
    }

    async fn cloudflare(&self) -> Result<Cloudflare> {
        let token = self.token().await?;
        Ok(Cloudflare::new(Some(self.api.as_str()), token.as_str()))
    }
}
