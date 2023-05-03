use primitives::{BlockHash, Header};

#[async_trait::async_trait]
pub trait HeaderStorage {
    async fn header_by_hash(&self, hash: BlockHash) -> Result<Option<Header>, anyhow::Error>;

    async fn save_header(&self, header: Header) -> Result<(), anyhow::Error>;
}
