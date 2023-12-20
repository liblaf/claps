use anyhow::Result;
use clap::{Args, Subcommand};

use claps::common::cmd::Run;
use reqwest::Url;

mod proxy;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Proxy(proxy::Cmd),
}

#[derive(Debug, Args)]
struct CommandArgs {
    #[arg(short, long, default_value = "http://127.0.0.1:9090")]
    url: Url,

    #[arg(short, long)]
    secret: Option<String>,
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        match self.sub_cmd {
            SubCmd::Proxy(cmd) => cmd.run().await,
        }
    }
}
