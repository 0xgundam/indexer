use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};
use ethers::{
    types::{Address, Bloom, Bytes, H256, H64, U256},
    utils::hex,
};
use primitives::{BlockHash, Header};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, Error as PgError, Pool, Postgres};

use crate::storage::traits::HeaderStorage;

pub struct PostgresHeaderStorage {
    pool: Pool<Postgres>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct DBHeader {
    hash: String,
    parent_hash: String,
    uncles_hash: String,
    author: String,
    state_root: String,
    transactions_root: String,
    receipts_root: String,
    number: BigDecimal,
    gas_used: BigDecimal,
    gas_limit: BigDecimal,
    extra_data: String,
    logs_bloom: String,
    timestamp: BigDecimal,
    difficulty: BigDecimal,
    size: BigDecimal,
    mix_hash: String,
    nonce: String,
    base_fee_per_gas: Option<BigDecimal>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum ParseDBHeaderError {
    #[error("Invalid hash")]
    HashInvalid,

    #[error("Invalid parent hash")]
    ParentHashInvalid,

    #[error("Invalid uncles hash")]
    UnclesHashInvalid,

    #[error("Invalid block number")]
    BlockNumberInvalid,

    #[error("Invalid timestamp")]
    TimestampInvalid,

    #[error("Invalid author")]
    AuthorInvalid,

    #[error("Invalid state root")]
    StateRootInvalid,

    #[error("Invalid transaction root")]
    TransactionRootInvalid,

    #[error("Invalid receipts root")]
    ReceiptsRootInvalid,

    #[error("Invalid gas used")]
    GasUsedInvalid,

    #[error("Invalid gas limit")]
    GasLimitInvalid,

    #[error("Invalid extra data")]
    ExtraDataInvalid,

    #[error("Invalid base fee per gas")]
    BaseFeePerGasInvalid,
}

impl TryFrom<&DBHeader> for Header {
    type Error = ParseDBHeaderError;

    fn try_from(header: &DBHeader) -> Result<Header, Self::Error> {
        let hash = match BlockHash::from_str(&header.hash) {
            Ok(hash) => hash,
            Err(_) => return Err(ParseDBHeaderError::HashInvalid),
        };
        let parent_hash = match BlockHash::from_str(&header.parent_hash) {
            Ok(parent_hash) => parent_hash,
            Err(_) => return Err(ParseDBHeaderError::ParentHashInvalid),
        };

        let uncles_hash = match BlockHash::from_str(&header.uncles_hash) {
            Ok(uncles_hash) => uncles_hash,
            Err(_) => return Err(ParseDBHeaderError::UnclesHashInvalid),
        };

        let author = match Address::from_str(&header.author) {
            Ok(author) => author,
            Err(_) => return Err(ParseDBHeaderError::AuthorInvalid),
        };

        let state_root = match H256::from_str(&header.state_root) {
            Ok(state_root) => state_root,
            Err(_) => return Err(ParseDBHeaderError::StateRootInvalid),
        };

        let transactions_root = match H256::from_str(&header.transactions_root) {
            Ok(transaction_root) => transaction_root,
            Err(_) => return Err(ParseDBHeaderError::TransactionRootInvalid),
        };

        let receipts_root = match H256::from_str(&header.receipts_root) {
            Ok(receipts_root) => receipts_root,
            Err(_) => return Err(ParseDBHeaderError::ReceiptsRootInvalid),
        };

        let number = match header.number.to_u64() {
            Some(number) => number,
            None => return Err(ParseDBHeaderError::BlockNumberInvalid),
        };

        let gas_used = match header.gas_used.to_u64() {
            Some(gas_used) => gas_used,
            None => return Err(ParseDBHeaderError::GasUsedInvalid),
        };

        let gas_limit = match header.gas_limit.to_u64() {
            Some(gas_limit) => gas_limit,
            None => return Err(ParseDBHeaderError::GasLimitInvalid),
        };

        let extra_data = match hex::decode(&header.extra_data) {
            Ok(extra_data) => Bytes::from(extra_data),
            Err(_) => return Err(ParseDBHeaderError::ExtraDataInvalid),
        };

        let logs_bloom = match Bloom::from_str(&header.logs_bloom) {
            Ok(logs_bloom) => logs_bloom,
            Err(_) => return Err(ParseDBHeaderError::ExtraDataInvalid),
        };

        let timestamp = match header.timestamp.to_u64() {
            Some(timestamp) => timestamp,
            None => return Err(ParseDBHeaderError::TimestampInvalid),
        };

        let difficulty = match U256::from_str(&header.difficulty.to_string()) {
            Ok(difficulty) => difficulty,
            Err(_) => return Err(ParseDBHeaderError::TimestampInvalid),
        };

        let size = match header.size.to_u64() {
            Some(size) => size,
            None => return Err(ParseDBHeaderError::TimestampInvalid),
        };

        let mix_hash = match H256::from_str(&header.mix_hash) {
            Ok(mix_hash) => mix_hash,
            Err(_) => return Err(ParseDBHeaderError::TimestampInvalid),
        };

        let nonce = match H64::from_str(&header.nonce) {
            Ok(nonce) => nonce,
            Err(_) => return Err(ParseDBHeaderError::TimestampInvalid),
        };

        let base_fee_per_gas = match &header.base_fee_per_gas {
            Some(base_fee_per_gas) => Some(base_fee_per_gas.to_u64().unwrap()),
            None => None,
        };

        Ok(Header {
            hash,
            parent_hash,
            uncles_hash,
            author,
            state_root,
            transactions_root,
            receipts_root,
            number,
            gas_used,
            gas_limit,
            extra_data,
            logs_bloom,
            timestamp,
            difficulty,
            size,
            mix_hash,
            nonce,
            base_fee_per_gas,
        })
    }
}

#[async_trait::async_trait]
impl HeaderStorage for PostgresHeaderStorage {
    async fn header_by_hash(&self, hash: BlockHash) -> Result<Option<Header>, anyhow::Error> {
        let header = query_as!(
            DBHeader,
            r#"
                    SELECT  
                        O.hash as "hash!",
                        O.parent_hash as "parent_hash!",
                        O.uncles_hash as "uncles_hash!",
                        O.author as "author!",
                        O.state_root as "state_root!",
                        O.transactions_root as "transactions_root!",
                        O.receipts_root as "receipts_root!",
                        O.number as "number!",
                        O.gas_used as "gas_used!",
                        O.gas_limit as "gas_limit!",
                        O.extra_data as "extra_data!",
                        O.logs_bloom as "logs_bloom!",
                        O.timestamp as "timestamp!",
                        O.difficulty as "difficulty!",
                        O.size as "size!",
                        O.mix_hash as "mix_hash!",
                        O.nonce as "nonce!",
                        O.base_fee_per_gas as "base_fee_per_gas!: Option<BigDecimal>"
                    FROM headers O
                    WHERE O.hash = $1
                "#,
            hash.to_string()
        )
        .fetch_one(&self.pool)
        .await;

        let header = match header {
            Ok(head) => Ok(Some(Header::try_from(&head)?)),
            Err(err) => match err {
                PgError::RowNotFound => Ok(None),
                _ => Err(err),
            },
        }?;

        Ok(header)
    }

    async fn save_header(&self, header: Header) -> Result<(), anyhow::Error> {
        let difficulty = header.difficulty.to_string();
        let difficulty = BigDecimal::from_str(&difficulty)?;
        query!(
            r#"
                INSERT INTO headers (
                    hash,
                    parent_hash,
                    uncles_hash,
                    author,
                    state_root,
                    transactions_root,
                    receipts_root,
                    number,
                    gas_used,
                    gas_limit,
                    extra_data,
                    logs_bloom,
                    timestamp,
                    difficulty,
                    size,
                    mix_hash,
                    nonce,
                    base_fee_per_gas
                ) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                        $11, $12, $13, $14, $15, $16, $17, $18)
                ON CONFLICT (hash) DO NOTHING
            "#,
            header.hash.to_string(),
            header.parent_hash.to_string(),
            header.uncles_hash.to_string(),
            header.author.to_string(),
            header.state_root.to_string(),
            header.transactions_root.to_string(),
            header.receipts_root.to_string(),
            BigDecimal::from_u64(header.number),
            BigDecimal::from_u64(header.gas_used),
            BigDecimal::from_u64(header.gas_limit),
            header.extra_data.to_string(),
            header.logs_bloom.to_string(),
            BigDecimal::from_u64(header.timestamp),
            difficulty,
            BigDecimal::from_u64(header.timestamp),
            header.mix_hash.to_string(),
            header.nonce.to_string(),
            header
                .base_fee_per_gas
                .and_then(|base_fee| BigDecimal::from_u64(base_fee))
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
