use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct Cmd {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    account: String,
    #[arg(from_global)]
    zone: String,
    #[arg(from_global)]
    name: Option<String>,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let client = crate::helper::client::accounts(
            self.api.as_str(),
            self.token.as_deref(),
            self.account.as_str(),
        )
        .await?;
        let client = client.cfd_tunnel();
        let cfd_tunnel = client
            .get(Some(self.name.unwrap_or_else(whoami::devicename).as_str()))
            .await?;
        let cfd_tunnel = cfd_tunnel.first().unwrap();
        let client = client.configurations(cfd_tunnel.id.as_str());
        let config = client.get().await?;
        println!("{}", config);
        Ok(())
    }
}
