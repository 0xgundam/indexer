use ethers_core::types::{Block, Bloom, Bytes, H160, H256, H64, U256};

use crate::{BlockHash, BlockNumber};

/// Slimmed down version of [Header]
pub struct Head {
    /// Block hash
    pub hash: BlockHash,
    /// Block number
    pub number: BlockNumber,
    /// Parent hash
    pub parent_hash: BlockHash,
    /// Timestamp
    pub timestamp: u64,
}

/// Block header
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Header {
    // Hash of the block
    pub hash: BlockHash,
    /// Hash of the parent
    pub parent_hash: BlockHash,
    /// Hash of the uncles
    pub uncles_hash: BlockHash,
    /// Miner/author's address
    pub author: H160,
    /// State root hash
    pub state_root: H256,
    /// Transactions root hash
    pub transactions_root: H256,
    /// Transactions receipts root hash
    pub receipts_root: H256,
    /// Block number
    pub number: BlockNumber,
    /// Gas Used
    pub gas_used: u64,
    /// Gas Limit
    pub gas_limit: u64,
    /// Extra data
    pub extra_data: Bytes,
    /// Logs bloom
    pub logs_bloom: Bloom,
    /// Timestamp
    pub timestamp: u64,
    /// Difficulty
    pub difficulty: U256,
    /// Size in bytes
    pub size: u64,
    /// Mix Hash
    pub mix_hash: H256,
    /// Nonce
    pub nonce: H64,
    /// Base fee per unit of gas (if past London)
    pub base_fee_per_gas: Option<u64>,
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidBlockError {
    /// Block missing hash
    #[error("Block missing hash")]
    MissingHash,

    /// Block missing author
    #[error("Block missing author")]
    MissingAuthor,

    /// Block missing number
    #[error("Block missing number")]
    MissingNumber,

    /// Block missing logs bloom
    #[error("Missing logs bloom")]
    MissingLogsBloom,

    /// Block missing size
    #[error("Missing size")]
    MissingSize,

    /// Block missing mix hash
    #[error("Missing mix hash")]
    MissingMixHash,

    /// Block missing nonce
    #[error("Missing nonce")]
    MissingNonce,
}

impl TryFrom<&Block<H256>> for Header {
    type Error = InvalidBlockError;

    fn try_from(block: &Block<H256>) -> Result<Self, InvalidBlockError> {
        let hash = match block.hash {
            Some(hash) => hash,
            None => return Err(InvalidBlockError::MissingHash),
        };

        let author = match block.author {
            Some(author) => author,
            None => return Err(InvalidBlockError::MissingAuthor),
        };

        let number = match block.number {
            Some(number) => number,
            None => return Err(InvalidBlockError::MissingNumber),
        };

        let logs_bloom = match block.logs_bloom {
            Some(logs_bloom) => logs_bloom,
            None => return Err(InvalidBlockError::MissingLogsBloom),
        };

        let base_fee_per_gas = match block.base_fee_per_gas {
            Some(base_fee_per_gas) => Some(base_fee_per_gas.as_u64()),
            None => None,
        };

        let size = match block.size {
            Some(size) => size.as_u64(),
            None => return Err(InvalidBlockError::MissingSize),
        };

        let mix_hash = match block.mix_hash {
            Some(mix_hash) => mix_hash,
            None => return Err(InvalidBlockError::MissingMixHash),
        };

        let nonce = match block.nonce {
            Some(nonce) => nonce,
            None => return Err(InvalidBlockError::MissingNonce),
        };

        Ok(Header {
            hash: hash,
            uncles_hash: block.uncles_hash,
            logs_bloom: logs_bloom,
            parent_hash: block.parent_hash,
            author: author,
            state_root: block.state_root,
            transactions_root: block.transactions_root,
            receipts_root: block.receipts_root,
            number: number.as_u64(),
            gas_used: block.gas_used.as_u64(),
            gas_limit: block.gas_limit.as_u64(),
            extra_data: block.extra_data.clone(),
            timestamp: block.timestamp.as_u64(),
            base_fee_per_gas: base_fee_per_gas,
            difficulty: block.difficulty,
            size: size,
            mix_hash: mix_hash,
            nonce: nonce,
        })
    }
}

impl From<&Header> for Head {
    fn from(header: &Header) -> Self {
        Head {
            hash: header.hash,
            number: header.number,
            parent_hash: header.parent_hash,
            timestamp: header.timestamp,
        }
    }
}
