use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::common::cmd::STYLES;
use claps::common::log::LogInit;

mod sub;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, about, styles = STYLES)]
pub struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd),
    Sub(sub::Cmd),
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            SubCmd::Complete(cmd) => cmd.run(Cmd::command()),
            SubCmd::Sub(cmd) => cmd.run().await,
        }
    }
}
