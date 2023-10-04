use crate::auth::GitHubUser;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    pub username: String,
    pub email: String,
    pub github_username: Option<String>,
    pub role: String,
    pub status: String,
}

impl User {
    pub fn new(username: String, email: String, github_username: String) -> Self {
        User {
            id: None,
            created_at: None,
            updated_at: None,
            username,
            email,
            github_username: Some(github_username),
            role: "user".to_string(),
            status: "active".to_string(),
        }
    }

    pub async fn upsert(self, pool: &PgPool) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO "user" (username, email, github_username)
            VALUES ($1, $2, $3)
            ON CONFLICT (username) DO UPDATE SET
                email = EXCLUDED.email,
                github_username = EXCLUDED.github_username,
                updated_at = now()
            RETURNING 
                id::TEXT,
                to_char("user".created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char("user".updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
                username, email, github_username, role, status
            ;
            "#,
            self.username,
            self.email,
            self.github_username.unwrap()
        )
        .fetch_one(pool)
        .await
    }

    pub async fn upsert_from_github_user(
        pool: &PgPool,
        github_user: GitHubUser,
    ) -> Result<Self, sqlx::Error> {
        let user = Self::new(
            github_user.login.clone(),
            github_user.email.clone(),
            github_user.login.clone(),
        );
        user.upsert(pool).await
    }
}
