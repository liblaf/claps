use serde::Deserialize;

mod create;
mod delete;
mod list;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub content: String,
    pub id: String,
    pub name: String,
    pub proxied: bool,
}
