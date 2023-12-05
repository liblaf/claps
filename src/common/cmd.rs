use anyhow::Result;

#[async_trait::async_trait]
pub trait Run {
    async fn run(self) -> Result<()>;
}
