use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::Args;

use claps::api::alist::Client;

use crate::cmd::GlobalArgs;

#[derive(Args)]
pub struct Cmd {
    #[arg(default_value("/"))]
    path: PathBuf,
    #[arg(short, long, default_value_t = 3)]
    depth: i32,
}

impl Cmd {
    pub async fn run(self, args: GlobalArgs) -> Result<()> {
        let client = args.client().await?;
        let tree = list(&client, self.path.as_path(), self.depth).await?;
        println!("{}", tree);
        Ok(())
    }
}

struct Tree {
    path: PathBuf,
    children: Vec<Tree>,
}

#[async_recursion::async_recursion]
async fn list(client: &Client, path: &Path, depth: i32) -> Result<Tree> {
    let mut tree = Tree {
        path: path.to_path_buf(),
        children: vec![],
    };
    if depth > 0 {
        let content = client.fs_list(Some(path.to_str().unwrap()), None).await?;
        tree.children.reserve_exact(content.len());
        for item in content.as_slice() {
            if item.is_dir {
                tree.children.push(
                    list(
                        client,
                        path.join(item.name.as_str()).join("").as_path(),
                        depth - 1,
                    )
                    .await?,
                );
            } else {
                tree.children.push(Tree {
                    path: path.join(item.name.as_str()).to_path_buf(),
                    children: vec![],
                });
            }
        }
    }
    Ok(tree)
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.display())?;
        for child in self.children.as_slice() {
            writeln!(f)?;
            write!(f, "{}", child)?;
        }
        Ok(())
    }
}
