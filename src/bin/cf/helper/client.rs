use anyhow::Result;
use claps::api::cloudflare::{accounts::ClientAccounts, Cloudflare};

pub async fn cloudflare(api: &str, token: Option<&str>) -> Result<Cloudflare> {
    let token = if let Some(token) = token {
        token.to_string()
    } else {
        claps::external::bw::get::notes("cloudflare.com").await?
    };
    Ok(Cloudflare::new(Some(api), token.as_str()))
}

pub async fn accounts(api: &str, token: Option<&str>, account: &str) -> Result<ClientAccounts> {
    let client = cloudflare(api, token).await?;
    Ok(client.accounts(account))
}

pub async fn zones(
    api: &str,
    token: Option<&str>,
    zone: &str,
) -> Result<claps::api::cloudflare::zones::ClientZones> {
    let client = cloudflare(api, token).await?;
    Ok(client.zones(zone))
}
