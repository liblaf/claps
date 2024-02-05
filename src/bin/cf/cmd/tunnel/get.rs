use anyhow::Result;
use clap::Args;

use super::AccountsArgsFromGlobal;

#[derive(Args)]
pub struct Cmd {
    #[command(flatten)]
    args: AccountsArgsFromGlobal,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let client = self.args.accounts().await?;
        let client = client.cfd_tunnel();
        let cfd_tunnel = client.get(Some(whoami::devicename().as_str())).await?;
        let cfd_tunnel = cfd_tunnel.first().unwrap();
        let client = client.configurations(cfd_tunnel.id.as_str());
        let config = client.get().await?;
        println!("{}", config);
        Ok(())
    }
}
