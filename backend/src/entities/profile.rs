use super::error::Result;
use crate::auth::GitHubUser;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Option<String>,
    pub user_id: Option<String>,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Profile {
    pub fn new(user_id: String, display_name: String, avatar_url: Option<String>) -> Self {
        Self {
            id: None,
            user_id: Some(user_id),
            display_name: Some(display_name),
            avatar_url,
            created_at: None,
            updated_at: None,
        }
    }

    pub async fn upsert(self, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            INSERT INTO profile (user_id, display_name, avatar_url)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id) DO UPDATE SET
                display_name = EXCLUDED.display_name,
                avatar_url = EXCLUDED.avatar_url,
                updated_at = NOW()
            RETURNING 
                id::TEXT, user_id::TEXT, display_name,
                to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
                avatar_url;
            "#,
            Uuid::parse_str(&self.user_id.unwrap()).ok(),
            self.display_name.unwrap(),
            self.avatar_url
        )
        .fetch_one(pool)
        .await?)
    }

    #[allow(dead_code)]
    pub async fn get_by_id(pool: &PgPool, id: String) -> Result<Self> {
        let uuid = Uuid::parse_str(&id).unwrap();

        Ok(sqlx::query_as!(
            Self,
            r#"
            SELECT 
                id::TEXT,
                user_id::TEXT, display_name,
                to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
                avatar_url
            FROM profile 
            WHERE id = $1"#,
            uuid
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_by_username(pool: &PgPool, username: String) -> Result<Self> {
        Ok(
            sqlx::query_file_as!(Self, "queries/profile_get_by_username.sql", username)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn upsert_from_github_user(
        pool: &PgPool,
        user_id: String,
        github_user: GitHubUser,
    ) -> Result<Self> {
        dbg!(&github_user);

        Self::new(user_id, github_user.name, Some(github_user.avatar_url))
            .upsert(&pool)
            .await
    }
}
