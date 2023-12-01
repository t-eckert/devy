use super::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub id: Option<String>,
    pub blog_id: Option<String>,

    pub url: Option<String>,
    pub github_id: Option<i64>,
    pub github_name: Option<String>,
    pub metadata: Option<Value>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoInput {
    pub blog_id: Uuid,

    pub url: String,
    pub github_id: i64,
    pub github_name: String,
    pub metadata: Value,
}

impl Repo {
    pub async fn get_by_url(pool: &PgPool, url: &str) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
                SELECT 
                    id::TEXT, blog_id::TEXT, url, github_id, github_name, metadata,
                    to_char(repo.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                    to_char(repo.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
                FROM "repo" WHERE url = $1;
                "#,
            url
        )
        .fetch_one(pool)
        .await?)
    }
}

pub struct RepoRepository {}

impl RepoRepository {
    pub async fn insert(pool: &PgPool, repo_input: RepoInput) -> Result<Repo> {
        Ok(sqlx::query_as!(
            Repo,
            r#"
                INSERT INTO "repo" (blog_id, url, github_id, github_name, metadata)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING 
                    id::TEXT, blog_id::TEXT, url, github_id, github_name, metadata,
                    to_char(repo.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                    to_char(repo.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
                "#,
            repo_input.blog_id,
            repo_input.url,
            repo_input.github_id,
            repo_input.github_name,
            repo_input.metadata
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_by_url(pool: &PgPool, url: &str) -> Result<Repo> {
        Ok(sqlx::query_as!(
            Repo,
            r#"
                SELECT 
                    id::TEXT, blog_id::TEXT, url, github_id, github_name, metadata,
                    to_char(repo.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                    to_char(repo.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
                FROM "repo" WHERE url = $1;
                "#,
            url
        )
        .fetch_one(pool)
        .await?)
    }
}
