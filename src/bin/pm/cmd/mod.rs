use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::common::cmd::{Run, STYLES};
use claps::common::log::LogInit;

mod search;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, styles = STYLES)]
pub(super) struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,

    #[command(flatten)]
    verbosity: Verbosity<InfoLevel>,
}

#[derive(Debug, Subcommand)]
enum SubCmd {
    Search(search::Cmd),
    Complete(claps::common::cmd::complete::Cmd<Cmd>),
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        self.verbosity.init();
        match self.sub_cmd {
            SubCmd::Search(cmd) => cmd.run().await,
            SubCmd::Complete(cmd) => cmd.run().await,
        }
    }
}
