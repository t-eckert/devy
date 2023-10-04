use crate::auth::GitHubUser;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub user_id: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Profile {
    pub fn new(user_id: String, display_name: String, avatar_url: Option<String>) -> Self {
        Self {
            user_id: Some(user_id),
            display_name: Some(display_name),
            avatar_url,
            created_at: None,
            updated_at: None,
        }
    }

    pub async fn upsert(self, pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO profile (user_id, display_name, avatar_url)
            VALUES ($1, $2, $3)
            RETURNING 
                user_id::TEXT, display_name,
                to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
                avatar_url;
            "#,
            Uuid::parse_str(&self.user_id.unwrap()).ok(),
            self.display_name.unwrap(),
            self.avatar_url
        )
        .fetch_one(pool)
        .await
    }

    pub async fn upsert_from_github_user(
        pool: &PgPool,
        github_user: GitHubUser,
    ) -> Result<Self, sqlx::Error> {
        Self::new(
            github_user.id.to_string(),
            github_user.name,
            Some(github_user.avatar_url),
        )
        .upsert(&pool)
        .await
    }
}
