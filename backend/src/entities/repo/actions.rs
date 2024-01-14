use sqlx::PgPool;

use super::*;

pub async fn upsert(pool: &PgPool, repo_input: RepoForUpsert) -> Result<Repo> {
    insert(pool, repo_input).await
}

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
