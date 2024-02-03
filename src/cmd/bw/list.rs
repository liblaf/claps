use anyhow::Result;

use crate::cmd::bw::types::Item;
use crate::common::log::LogResult;

pub async fn items(search: Option<&str>) -> Result<Vec<Item>> {
    let mut args = vec!["items"];
    if let Some(search) = search {
        args.push("--search");
        args.push(search);
    }
    let output = crate::cmd::bw::list(args).await?;
    serde_json::from_slice(output.as_slice()).log()
}
