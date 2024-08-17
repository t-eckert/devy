use crate::db::{error::Result, Database};
use uuid::Uuid;
use super::Follow;

pub async fn insert(db: &Database, follow: Follow) -> Result<()> {
    sqlx::query_as!(Follow, r#"
        INSERT INTO "follow" (profile_id, blog_id)
        VALUES ($1, $2)
            ON CONFLICT DO NOTHING;
        "#, follow.profile_id, follow.blog_id).execute(db).await?;

    Ok(())
}

pub async fn get_by_profile_id(db: &Database, profile_id: Uuid) -> Result<Vec<Follow>> {
    Ok(sqlx::query_as!(Follow, r#"
        SELECT profile_id, blog_id FROM "follow"
        WHERE profile_id = $1;
        "#, profile_id).fetch_all(db).await?)
}

pub async fn delete(db: &Database, follow: Follow) -> Result<()> {
    sqlx::query_as!(Follow, r#"
        DELETE FROM "follow"
        WHERE profile_id = $1 AND blog_id = $2;
        "#, follow.profile_id, follow.blog_id).execute(db).await?;

    Ok(())
}
