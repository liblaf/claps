use anyhow::Result;
use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};

use claps::common::cmd::STYLES;
use claps::common::log::LogInit;

mod delete;
mod install;
mod list;
mod update;

#[derive(Parser)]
#[command(name = env!("CARGO_BIN_NAME"), version, author, about, styles = STYLES)]
pub struct Cmd {
    #[command(subcommand)]
    sub_cmd: SubCmd,
    #[command(flatten)]
    args: GlobalArgs,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

#[derive(Subcommand)]
enum SubCmd {
    Complete(claps::common::cmd::complete::Cmd),
    Delete(delete::Cmd),
    Install(install::Cmd),
    List(list::Cmd),
    Update(update::Cmd),
}

#[derive(Args)]
struct GlobalArgs {
    #[arg(short, long, env, global = true)]
    name: Option<String>,
    #[arg(short, long, env, global = true)]
    token: Option<String>,
    #[arg(
        short,
        long,
        env,
        default_value = "919b04037636d3b4bbc0af135eaccdfa",
        global = true
    )]
    zone: String,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        self.verbose.init();
        match self.sub_cmd {
            SubCmd::Complete(cmd) => cmd.run(Cmd::command()),
            SubCmd::Delete(cmd) => cmd.run(self.args).await,
            SubCmd::Install(cmd) => cmd.run().await,
            SubCmd::List(cmd) => cmd.run(self.args).await,
            SubCmd::Update(cmd) => cmd.run(self.args).await,
        }
    }
}

impl GlobalArgs {
    fn name(&self) -> Result<String> {
        if let Some(name) = &self.name {
            return Ok(name.to_string());
        }
        Ok(format!("{}.ddns.liblaf.me", whoami::hostname()))
    }

    async fn token(&self) -> Result<String> {
        if let Some(token) = self.token.as_deref() {
            return Ok(token.to_string());
        }
        claps::cmd::bw::get::notes("CLOUDFLARE_TOKEN").await
    }
}
