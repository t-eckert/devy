use super::error::Result;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    pub profile_id: Uuid,
    pub post_id: Uuid,
}

impl Like {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLike {
    pub profile_id: Uuid,
    pub post_id: Uuid,
}

pub struct LikeRepository {}

impl LikeRepository {
    pub async fn upsert(pool: &PgPool, like: NewLike) -> Result<Like> {
        Ok(sqlx::query_file_as!(
            Like,
            "queries/like_upsert.sql",
            like.profile_id,
            like.post_id
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_by_username(pool: &PgPool, username: String) -> Result<Vec<Like>> {
        Ok(
            sqlx::query_file_as!(Like, "queries/like_get_by_username.sql", username)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn delete(pool: &PgPool, profile_id: Uuid, post_id: Uuid) -> Result<()> {
        let _ = sqlx::query_file_as!(Like, "queries/like_delete.sql", profile_id, post_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
