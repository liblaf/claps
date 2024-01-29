use std::fmt::Display;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Args;

use claps::api::alist::Client;

#[derive(Debug, Args)]
pub(super) struct Cmd {
    #[arg(default_value = "/")]
    path: PathBuf,
    #[arg(short, long, default_value = "1")]
    depth: i64,
    #[arg(short, long)]
    refresh: bool,
}

impl Cmd {
    pub async fn run(self, args: crate::cmd::CommonArgs) -> Result<()> {
        let mut client = Client::new(args.url.as_str());
        client
            .auth_login(args.username()?.as_str(), args.password()?.as_str())
            .await?;
        println!(
            "{}",
            tree(&client, self.path.as_path(), self.depth, self.refresh).await?
        );
        Ok(())
    }
}

#[async_recursion::async_recursion]
async fn tree(
    client: &Client,
    path: &'async_recursion Path,
    depth: i64,
    refresh: bool,
) -> Result<Tree> {
    let data = client
        .fs_list(Some(path.to_str().unwrap()), None, Some(refresh))
        .await
        .unwrap();
    let children = futures::future::join_all(
        data.content
            .unwrap_or_default()
            .iter()
            .map(|c| async {
                let path = path.join(c.name.as_str());
                if depth > 1 && c.is_dir {
                    return tree(client, path.as_path(), depth - 1, refresh).await.ok();
                }
                Some(Tree::new(path.as_path(), c.is_dir))
            })
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();
    Ok(Tree::new(path, true).with_children(children))
}

struct Tree {
    path: PathBuf,
    is_dir: bool,
    children: Vec<Tree>,
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())?;
        if self.is_dir {
            write!(f, "/")?;
        }
        writeln!(f)?;
        for child in self.children.iter() {
            child.fmt(f)?;
        }
        Ok(())
    }
}

impl Tree {
    fn new(path: &Path, is_dir: bool) -> Self {
        Self {
            path: path.to_path_buf(),
            is_dir,
            children: Vec::new(),
        }
    }

    fn with_children<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        self.children = children.into_iter().collect();
        self
    }
}
