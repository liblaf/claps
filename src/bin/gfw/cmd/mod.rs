use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::common::cmd::STYLES;
use claps::common::log::LogInit;

mod ip;
mod sub;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, about, styles = STYLES)]
pub struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
    #[arg(long)]
    no_proxy: bool,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd),
    IP(ip::Cmd),
    Sub(sub::Cmd),
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        self.verbose.init();
        if self.no_proxy {
            for (key, _) in std::env::vars() {
                if key.to_lowercase().contains("proxy") {
                    std::env::remove_var(key);
                }
            }
        }
        match self.sub_cmd {
            SubCmd::Complete(cmd) => cmd.run(Cmd::command()),
            SubCmd::IP(cmd) => cmd.run().await,
            SubCmd::Sub(cmd) => cmd.run().await,
        }
    }
}
