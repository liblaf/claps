use anyhow::Result;
use clap::{Args, Subcommand};
use claps::api::cloudflare::accounts::Accounts;

use super::CloudflareArgs;

mod get;
mod put;

#[derive(Args)]
pub struct Cmd {
    #[command(subcommand)]
    cmd: SubCmd,
    #[command(flatten)]
    args: AccountsArgs,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        match self.cmd {
            SubCmd::Get(cmd) => cmd.run().await,
            SubCmd::Put(cmd) => cmd.run().await,
        }
    }
}

#[derive(Subcommand)]
enum SubCmd {
    Get(get::Cmd),
    Put(put::Cmd),
}

#[derive(Args)]
struct AccountsArgs {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(
        long,
        default_value = "7ad40aa54c5d9453abe45eeb3c6643de",
        global = true
    )]
    account: String,
}

#[derive(Args)]
struct AccountsArgsFromGlobal {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    account: String,
}

impl AccountsArgsFromGlobal {
    fn args(&self) -> CloudflareArgs {
        CloudflareArgs {
            api: self.api.to_string(),
            token: self.token.to_owned(),
        }
    }

    async fn accounts(&self) -> Result<Accounts> {
        let cloudflare = self.args().cloudflare().await?;
        Ok(cloudflare.accounts(self.account.as_str()))
    }
}
