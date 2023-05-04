use std::str::FromStr;

use crate::storage::traits::HeadStorage;
use primitives::{BlockHash, Head};
use serde::{Deserialize, Serialize};
use sqlx::{query_as, types::BigDecimal, Error as PgError, Pool, Postgres};

use bigdecimal::ToPrimitive;

use super::header::ParseDBHeaderError;

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

impl TryFrom<&DBHead> for Head {
    type Error = ParseDBHeaderError;

    fn try_from(head: &DBHead) -> Result<Head, Self::Error> {
        let hash = match BlockHash::from_str(&head.hash) {
            Ok(hash) => hash,
            Err(_) => return Err(ParseDBHeaderError::HashInvalid),
        };

        let number = match head.number.to_u64() {
            Some(num) => num,
            None => return Err(ParseDBHeaderError::BlockNumberInvalid),
        };

        let parent_hash = match BlockHash::from_str(&head.parent_hash) {
            Ok(parent_hash) => parent_hash,
            Err(_) => return Err(ParseDBHeaderError::ParentHashInvalid),
        };

        let timestamp = match head.timestamp.to_u64() {
            Some(timestamp) => timestamp,
            None => return Err(ParseDBHeaderError::TimestampInvalid),
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
