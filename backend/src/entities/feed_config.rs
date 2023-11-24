use super::error::Result;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedConfig {
    pub id: Uuid,
    pub name: Option<String>,
}

pub struct FeedConfigRepository {}

impl FeedConfigRepository {
    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<FeedConfig> {
        Ok(sqlx::query_as!(
            FeedConfig,
            r#"
            SELECT 
                id,
                name
            FROM "feed_config"
            WHERE id = $1;
            "#,
            id
        )
        .fetch_one(pool)
        .await?)
    }
}
