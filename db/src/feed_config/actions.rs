use crate::{Database, Result};
use entities::FeedConfig;
use uuid::Uuid;

pub async fn get_by_id(db: &Database, id: Uuid) -> Result<FeedConfig> {
    Ok(
        sqlx::query_file_as!(FeedConfig, "src/feed_config/queries/get_by_id.sql", id)
            .fetch_one(db)
            .await?,
    )
}
