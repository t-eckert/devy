use crate::db;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    pub profile_id: Uuid,
    pub blog_id: Uuid,
}

pub struct FollowRepository;

impl FollowRepository {
    pub async fn insert(db_conn: &db::Conn, follow: Follow) -> db::Result<()> {
        sqlx::query_as!(
            Follow,
            r#"
            INSERT INTO "follow" (profile_id, blog_id)
            VALUES ($1, $2)
                ON CONFLICT DO NOTHING;
            "#,
            follow.profile_id,
            follow.blog_id
        )
        .execute(db_conn)
        .await?;

        Ok(())
    }

    pub async fn get_by_profile_id(
        db_conn: &db::Conn,
        profile_id: Uuid,
    ) -> db::Result<Vec<Follow>> {
        Ok(sqlx::query_as!(
            Follow,
            r#"
            SELECT profile_id, blog_id FROM "follow"
            WHERE profile_id = $1;
            "#,
            profile_id
        )
        .fetch_all(db_conn)
        .await?)
    }

    pub async fn delete(db_conn: &db::Conn, follow: Follow) -> db::Result<()> {
        sqlx::query_as!(
            Follow,
            r#"
            DELETE FROM "follow"
            WHERE profile_id = $1 AND blog_id = $2;
            "#,
            follow.profile_id,
            follow.blog_id
        )
        .execute(db_conn)
        .await?;

        Ok(())
    }
}
