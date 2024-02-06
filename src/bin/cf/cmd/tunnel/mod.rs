use anyhow::Result;
use clap::{Args, Subcommand};

mod get;
mod load_balancer;
mod put;

#[derive(Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
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
