use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn insert(pool: &PgPool, repo_input: RepoForUpsert) -> Result<Repo> {
    Ok(sqlx::query_file_as!(
        Repo,
        "src/entities/repo/queries/insert.sql",
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
    Ok(
        sqlx::query_file_as!(Repo, "src/entities/repo/queries/get_by_url.sql", url)
            .fetch_one(pool)
            .await?,
    )
}
