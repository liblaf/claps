use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct Cmd {
    #[arg(from_global)]
    api: String,
    #[arg(from_global)]
    token: Option<String>,
    #[arg(from_global)]
    zone: String,
    #[arg(from_global)]
    name: Option<String>,
}

impl Cmd {
    pub async fn run(self) -> Result<()> {
        let name = format!(
            "{}.ddns.liblaf.me",
            self.name.unwrap_or_else(whoami::hostname)
        );
        let client = crate::helper::client::zones(
            self.api.as_str(),
            self.token.as_deref(),
            self.zone.as_str(),
        )
        .await?;
        let client = client.dns_records();
        let records = client.get(Some(name.as_str())).await?;
        let tasks = records
            .iter()
            .map(|record| client.delete(record.id.as_str(), Some(record)))
            .collect::<Vec<_>>();
        futures::future::try_join_all(tasks).await?;
        Ok(())
    }
}
