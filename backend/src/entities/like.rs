use super::error::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    pub profile_id: Option<String>,
    pub post_id: Option<String>,
}

impl Like {
    pub fn new(profile_id: String, post_id: String) -> Self {
        Like {
            profile_id: Some(profile_id),
            post_id: Some(post_id),
        }
    }

    pub fn profile_uuid(&self) -> Result<Uuid> {
        match &self.profile_id {
            Some(id) => {
                Uuid::try_parse(&id).map_err(|_| Error::malformed("Like.profile_id is not UUID."))
            }
            None => Err(Error::malformed("Like is missing profile_id.")),
        }
    }

    pub fn post_uuid(&self) -> Result<Uuid> {
        match &self.post_id {
            Some(id) => {
                Uuid::try_parse(&id).map_err(|_| Error::malformed("Like.post_id is not UUID."))
            }
            None => Err(Error::malformed("Like is missing post_id.")),
        }
    }

    pub async fn upsert(self, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            INSERT INTO "like" (profile_id, post_id)
            VALUES ($1, $2)
                ON CONFLICT (profile_id, post_id)
                DO UPDATE SET profile_id = $1, post_id = $2
            RETURNING profile_id::TEXT, post_id::TEXT;
            "#,
            self.profile_uuid()?,
            self.post_uuid()?
        )
        .fetch_one(pool)
        .await?)
    }

    // Returns the ids of all posts liked by a user.
    pub async fn get_post_ids_by_username(pool: &PgPool, username: String) -> Result<Vec<String>> {
        // TODO simplify this to just return the list of post_ids from the SQL query instead of
        // throwing away data in the map.
        sqlx::query_as!(
            Self,
            r#"
            SELECT profile_id::TEXT, post_id::TEXT
            FROM "like" LEFT JOIN (
                SELECT 
            profile.id, username
            FROM "profile" LEFT JOIN "user"
            ON user_id="user".id
            ) AS "profile" ON profile_id="profile".id
            WHERE username = $1;
            "#,
            username
        )
        .fetch_all(pool)
        .await
        .map_err(|x| x.into())
        .map(|likes| {
            likes
                .into_iter()
                .map(|like| like.post_id.unwrap())
                .collect()
        })
    }

    pub async fn delete(self, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            DELETE FROM "like"
            WHERE profile_id = $1 AND post_id = $2
            RETURNING profile_id::TEXT, post_id::TEXT;
            "#,
            self.profile_uuid()?,
            self.post_uuid()?
        )
        .fetch_one(pool)
        .await?)
    }
}
