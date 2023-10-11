use super::error::EntityError;
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

    pub fn profile_uuid(&self) -> Result<Uuid, EntityError> {
        match &self.profile_id {
            Some(id) => Uuid::try_parse(&id)
                .map_err(|_| EntityError::malformed("Like.profile_id is not UUID.")),
            None => Err(EntityError::malformed("Like is missing profile_id.")),
        }
    }

    pub fn post_uuid(&self) -> Result<Uuid, EntityError> {
        match &self.post_id {
            Some(id) => Uuid::try_parse(&id)
                .map_err(|_| EntityError::malformed("Like.post_id is not UUID.")),
            None => Err(EntityError::malformed("Like is missing post_id.")),
        }
    }

    pub async fn upsert(self, pool: &PgPool) -> Result<Self, EntityError> {
        sqlx::query_as!(
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
        .await
        .map_err(|x| x.into())
    }

    pub async fn delete(self, pool: &PgPool) -> Result<Self, EntityError> {
        sqlx::query_as!(
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
        .await
        .map_err(|x| x.into())
    }
}
