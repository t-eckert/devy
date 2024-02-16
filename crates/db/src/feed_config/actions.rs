use sqlx::PgPool;
use uuid::Uuid;

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<FeedConfig> {
    Ok(sqlx::query_file_as!(
        FeedConfig,
        "src/entities/feed_config/queries/get_by_id.sql",
        id
    )
    .fetch_one(pool)
    .await?)
}
