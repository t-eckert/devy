use serde::{Deserialize, Serialize};

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

    pub async fn upsert(self, pool: &PgPool) -> Result<Self, sqlx::Error> {
        // Profile and post ids must be present and valid uuids.
        let (profile_id, post_id) = match (self.profile_id, self.post_id) {
            (Some(profile_id), Some(post_id)) => {
                match (
                    Uuid::parse_str(profile_id.as_str()),
                    Uuid::parse_str(post_id.as_str()),
                ) {
                    (Ok(profile_id), Ok(post_id)) => (profile_id, post_id),
                    _ => return None,
                }
            }
            _ => return None,
        };

        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO "like" (profile_id, post_id)
            VALUES ($1, $2)
                ON CONFLICT (profile_id, post_id)
                DO UPDATE SET profile_id = $1, post_id = $2
            RETURNING profile_id::TEXT, post_id::TEXT;
            "#,
            profile_id,
            post_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete(self, pool: &PgPool) -> Option<Like> {
        let (profile, post) = match (
            Uuid::parse_str(self.profile_id.as_str()),
            Uuid::parse_str(self.post_id.as_str()),
        ) {
            (Ok(profile), Ok(post)) => (profile, post),
            _ => return None,
        };

        sqlx::query_as!(
            Self,
            r#"
            DELETE FROM "like"
            WHERE profile_id = $1 AND post_id = $2
            RETURNING profile_id::TEXT, post_id::TEXT;
            "#,
            profile,
            post
        )
        .fetch_one(&mut *db)
        .await
    }
}
