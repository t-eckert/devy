use crate::db::Result;
use crate::entities::Like;
use sqlx::types::Uuid;
use sqlx::PgPool;

pub async fn upsert(pool: &PgPool, profile_id: Uuid, post_id: Uuid) -> Result<Like> {
    Ok(
        sqlx::query_file_as!(Like, "src/db/like/queries/upsert.sql", profile_id, post_id)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_by_username(pool: &PgPool, username: String) -> Result<Vec<Like>> {
    Ok(
        sqlx::query_file_as!(Like, "src/db/like/queries/get_by_username.sql", username)
            .fetch_all(pool)
            .await?,
    )
}

pub async fn delete(pool: &PgPool, profile_id: Uuid, post_id: Uuid) -> Result<()> {
    let _ = sqlx::query_file_as!(Like, "src/db/like/queries/delete.sql", profile_id, post_id)
        .execute(pool)
        .await?;

    Ok(())
}
