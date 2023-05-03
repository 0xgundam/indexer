use std::str::FromStr;

use crate::storage::traits::HeadStorage;
use primitives::{BlockHash, Head};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, types::BigDecimal, Error as PgError, Pool, Postgres};

use bigdecimal::ToPrimitive;

pub struct PostgresHeadStorage {
    pool: Pool<Postgres>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct DBHead {
    hash: String,
    number: BigDecimal,
    parent_hash: String,
    timestamp: BigDecimal,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseDBHeadError {
    #[error("Invalid hash")]
    InvalidHash,

    #[error("Invalid parent hash")]
    InvalidParentHash,

    #[error("Invalid block number")]
    InvalidBlockNumber,

    #[error("Invalid timestamp")]
    InvalidTimestamp,
}

impl TryFrom<&DBHead> for Head {
    type Error = ParseDBHeadError;

    fn try_from(head: &DBHead) -> Result<Head, Self::Error> {
        let hash = match BlockHash::from_str(&head.hash) {
            Ok(hash) => hash,
            Err(_) => return Err(ParseDBHeadError::InvalidHash),
        };

        let number = match head.number.to_u64() {
            Some(num) => num,
            None => return Err(ParseDBHeadError::InvalidBlockNumber),
        };

        let parent_hash = match BlockHash::from_str(&head.parent_hash) {
            Ok(parent_hash) => parent_hash,
            Err(_) => return Err(ParseDBHeadError::InvalidParentHash),
        };

        let timestamp = match head.timestamp.to_u64() {
            Some(timestamp) => timestamp,
            None => return Err(ParseDBHeadError::InvalidTimestamp),
        };

        Ok(Head {
            hash: hash,
            parent_hash: parent_hash,
            timestamp: timestamp,
            number: number,
        })
    }
}

#[async_trait::async_trait]
impl HeadStorage for PostgresHeadStorage {
    async fn head(&self) -> Result<Option<Head>, anyhow::Error> {
        let head = query_as!(
            DBHead,
            "SELECT hash, number, parent_hash, timestamp FROM headers ORDER BY number DESC LIMIT 1"
        )
        .fetch_one(&self.pool)
        .await;

        let head = match head {
            Ok(head) => Ok(Some(Head::try_from(&head)?)),
            Err(err) => match err {
                PgError::RowNotFound => Ok(None),
                _ => Err(err),
            },
        }?;

        Ok(head)
    }
}
