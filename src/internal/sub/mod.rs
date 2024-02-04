use anyhow::Result;
use reqwest::{IntoUrl, Url};

use crate::common::log::LogResult;
use crate::external::bw::types::{Item, ItemLogin};

mod info;

use info::UserInfo;

#[derive(Debug)]
pub struct Sub {
    pub name: String,
    pub url: Option<Url>,
    pub user_info: Option<UserInfo>,
}

static HOST2NAME: phf::Map<&str, &str> = phf::phf_map! {
    "apiv1.v27qae.com"  => "FlyingBird",
    "apiv2.lipulai.com" => "FastLink",
    "www.sub-nthu.com"  => "NTHU.CC",
};

impl<U> From<U> for Sub
where
    U: IntoUrl,
{
    fn from(url: U) -> Self {
        let url = url.into_url().log().unwrap();
        Self {
            name: HOST2NAME
                .get(url.host_str().unwrap())
                .unwrap_or(&"")
                .to_string(),
            url: Some(url),
            user_info: None,
        }
    }
}

impl From<ItemLogin> for Sub {
    fn from(login: ItemLogin) -> Self {
        Self {
            name: login.name,
            url: login.notes.map(|notes| notes.parse().log().unwrap()),
            user_info: None,
        }
    }
}

pub async fn get_subs() -> Result<Vec<Sub>> {
    let folder = crate::external::bw::get::folder("the Great Wall").await?;
    let items = crate::external::bw::list::items(None, Some(folder.id.as_str())).await?;
    let items = items.into_iter().filter_map(|item| match item {
        Item::Login(item) => Some(item),
        Item::SecureNote(_) => None,
    });
    let subs = items.map(Sub::from);
    Ok(subs.collect())
}
