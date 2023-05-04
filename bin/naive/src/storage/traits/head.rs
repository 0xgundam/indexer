use primitives::Head;

#[async_trait::async_trait]
pub trait HeadStorage {
    async fn head(&self) -> Result<Option<Head>, anyhow::Error>;
}
