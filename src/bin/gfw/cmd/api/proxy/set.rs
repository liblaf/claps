use anyhow::Result;
use clap::Args;
use inquire::Select;

use claps::api::clash::Client;
use claps::common::cmd::Run;

use crate::cmd::api::CommandArgs;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[command(flatten)]
    common: CommandArgs,

    name: Option<String>,
}

#[async_trait::async_trait]
impl Run for Cmd {
    async fn run(self) -> Result<()> {
        let client = Client::new(self.common.url, self.common.secret)?;
        let proxies = client.proxies().await?;
        let proxy = proxies.get("PROXY").unwrap();
        let all = proxy.all.as_deref().unwrap();
        let all = if let Some(name) = self.name {
            all.iter()
                .filter(|n| n.contains(name.as_str()))
                .map(|n| n.to_string())
                .collect()
        } else {
            all.to_vec()
        };
        let name = if all.len() == 1 {
            all.first().unwrap()
        } else {
            let options: Vec<_> = all
                .iter()
                .map(|name| crate::cmd::api::proxy::pretty(&proxies, name))
                .collect();
            let select = Select::new("Select Proxy", options.to_owned()).prompt()?;
            let select = options
                .iter()
                .position(|pretty| pretty.as_str() == select.as_str())
                .unwrap();
            all.get(select).unwrap()
        };
        client.proxy_set("PROXY", name.as_str()).await?;
        Ok(())
    }
}
