use anyhow::Result;
use clap::Args;
use inquire::Select;

use claps::api::clash::Client;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    name: Option<String>,
}

impl Cmd {
    pub async fn run(&self, args: crate::cmd::proxy::CommonArgs) -> Result<()> {
        let client = Client::new(args.url, args.secret)?;
        let proxies = client.proxies().await?;
        let proxies = proxies.get("PROXY").unwrap().all.as_deref().unwrap();
        let pretties = crate::cmd::proxy::pretty(&client, "PROXY").await?;
        let mut filtered_proxies = vec![];
        let mut filtered_pretties = vec![];
        for (proxy, pretty) in proxies.iter().zip(pretties.iter()) {
            if let Some(name) = self.name.as_deref() {
                if !proxy.contains(name) {
                    continue;
                }
            }
            filtered_proxies.push(proxy);
            filtered_pretties.push(pretty);
        }
        let proxies = filtered_proxies;
        let pretties = filtered_pretties;
        let name = if proxies.len() == 1 {
            proxies.first().unwrap()
        } else {
            let select = Select::new("Select Proxy", pretties.to_owned()).prompt()?;
            let select = pretties
                .iter()
                .position(|pretty| pretty.as_str() == select.as_str())
                .unwrap();
            proxies.get(select).unwrap()
        };
        client.proxy_set("PROXY", name.as_str()).await?;
        Ok(())
    }
}
