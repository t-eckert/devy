use super::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub id: Uuid,
    pub url: String,
    pub blog_id: Uuid,

    pub github_id: i64,
    pub github_name: String,
    pub metadata: Option<Value>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRepo {
    pub blog_id: Uuid,

    pub url: String,
    pub github_id: i64,
    pub github_name: String,
    pub metadata: Value,
}

pub struct RepoRepository {}

impl RepoRepository {
    pub async fn insert(pool: &PgPool, repo_input: NewRepo) -> Result<Repo> {
        Ok(sqlx::query_as!(
            Repo,
            r#"
                INSERT INTO "repo" (blog_id, url, github_id, github_name, metadata)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING 
                    id, blog_id, url, github_id, github_name, metadata,
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
                    id, blog_id, url, github_id, github_name, metadata,
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
