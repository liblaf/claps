use clap::{Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::common::cmd::{Run, STYLES};
use claps::common::log::LogInit;

mod ip;
mod proxy;
mod sub;

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
    Proxy(proxy::Cmd),
    Complete(claps::common::cmd::complete::Cmd<Cmd>),
    IP(ip::Cmd),
    Sub(sub::Cmd),
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> anyhow::Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            SubCmd::Proxy(cmd) => cmd.run().await,
            SubCmd::Complete(cmd) => cmd.run().await,
            SubCmd::IP(cmd) => cmd.run().await,
            SubCmd::Sub(cmd) => cmd.run().await,
        }
    }
}
