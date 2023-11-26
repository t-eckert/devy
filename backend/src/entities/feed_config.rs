use super::error::Result;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedConfig {
    pub id: Uuid,
    pub name: String,
}

impl FeedConfig {
    pub fn new(id: Uuid, name: String) -> Self {
        Self { id, name }
    }
}

pub struct FeedConfigRepository {}

impl FeedConfigRepository {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<FeedConfig> {
        Ok(
            sqlx::query_file_as!(FeedConfig, "queries/feed_config_get_by_id.sql", id)
                .fetch_one(pool)
                .await?,
        )
    }
}
