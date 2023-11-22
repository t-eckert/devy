use super::error::{Error, Result};
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

    pub async fn upsert(self, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_file_as!(
            Self,
            "queries/user_upsert.sql",
            self.username,
            self.email,
            self.github_username.unwrap()
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn upsert_from_github_user(pool: &PgPool, github_user: GitHubUser) -> Result<Self> {
        let user = Self::new(
            github_user
                .login
                .clone()
                .ok_or(Error::Malformed("GitHub User missing login".to_string()))?,
            github_user
                .email
                .clone()
                .ok_or(Error::Malformed("GitHub User missing email".to_string()))?,
            github_user
                .login
                .clone()
                .ok_or(Error::Malformed("GitHub User missing login".to_string()))?,
        );
        user.upsert(pool).await
    }

    pub async fn get_by_username(pool: &PgPool, username: String) -> Result<Self> {
        Ok(
            sqlx::query_file_as!(User, "queries/user_get_by_username.sql", username)
                .fetch_one(pool)
                .await?,
        )
    }

    #[allow(dead_code)]
    pub async fn get_by_github_username(pool: &PgPool, github_username: String) -> Result<Self> {
        Ok(sqlx::query_file_as!(
            User,
            "queries/user_get_by_github_username.sql",
            github_username
        )
        .fetch_one(pool)
        .await?)
    }
}
