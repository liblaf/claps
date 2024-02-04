use anyhow::Result;

use crate::common::log::LogResult;
use crate::external::bw::types::Item;

pub async fn items(search: Option<&str>, folderid: Option<&str>) -> Result<Vec<Item>> {
    let mut args = vec!["items"];
    if let Some(search) = search {
        args.push("--search");
        args.push(search);
    }
    if let Some(folderid) = folderid {
        args.push("--folderid");
        args.push(folderid);
    }
    let output = crate::external::bw::list(args).await?;
    serde_json::from_slice(output.as_slice()).log()
}
