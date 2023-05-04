use ethers_core::types::H256;

mod header;

/// A block hash
pub type BlockHash = H256;

/// A block number
pub type BlockNumber = u64;

/// A transaction hash
pub type TxHash = H256;

pub use header::{Head, Header, InvalidBlockError};
