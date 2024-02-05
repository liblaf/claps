use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::common::log::LogInit;

mod dns;
mod tunnel;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, about, styles = claps::common::cmd::STYLES)]
pub struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
    #[arg(long, default_value = claps::api::cloudflare::API, global(true))]
    api: String,
    #[arg(short, long, global(true))]
    token: Option<String>,
    #[arg(
        short,
        long,
        default_value = "7ad40aa54c5d9453abe45eeb3c6643de",
        global(true)
    )]
    account: String,
    #[arg(
        short,
        long,
        default_value = "919b04037636d3b4bbc0af135eaccdfa",
        global(true)
    )]
    zone: String,
    #[arg(short, long, global(true))]
    name: Option<String>,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd),
    Dns(dns::Cmd),
    Tunnel(tunnel::Cmd),
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            SubCmd::Complete(cmd) => cmd.run(Cmd::command()),
            SubCmd::Dns(cmd) => cmd.run().await,
            SubCmd::Tunnel(cmd) => cmd.run().await,
        }
    }
}
