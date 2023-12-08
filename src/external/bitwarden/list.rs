use std::ffi::OsStr;

use anyhow::Result;

use crate::common::log::LogResult;
use crate::external::bitwarden::types::Item;

fn list<I, S>(args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    crate::external::bitwarden::bw("list", args)
}

pub fn items(search: &str, folderid: &str) -> Result<Vec<Item>> {
    let mut args = vec!["items".to_string()];
    if !search.is_empty() {
        args.push(format!("--search={}", search));
    }
    if !folderid.is_empty() {
        args.push(format!("--folderid={}", folderid));
    }
    let items = list(args)?;
    serde_json::from_str(&items).log()
}
