use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

use super::*;

pub async fn upsert(pool: &PgPool, repo_input: RepoForUpsert) -> Result<Repo> {
    Ok(sqlx::query_file_as!(
        Repo,
        "src/entities/repo/queries/upsert.sql",
        repo_input.blog_id,
        repo_input.url,
        repo_input.github_id,
        repo_input.github_name,
    )
    .fetch_one(pool)
    .await?)
}

pub async fn set_metadata(pool: &PgPool, repo_id: Uuid, metadata: Value) -> Result<Repo> {
    Ok(sqlx::query_file_as!(
        Repo,
        "src/entities/repo/queries/set_metadata.sql",
        repo_id,
        metadata,
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

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::uuid;

    #[sqlx::test]
    async fn test_upsert_with_new(pool: PgPool) {
        let blog_id = uuid!("9fb93736-aee0-4eda-a960-f8c0ec5870ab");
        let url = "https://github.com/t-eckert/devy-test-repo".to_string();
        let github_id = 123456789;
        let github_name = "devy-test-repo".to_string();

        let repo = RepoForUpsert::new(
            blog_id.clone(),
            url.clone(),
            github_id.clone(),
            github_name.clone(),
        );

        let actual = upsert(&pool, repo).await.unwrap();

        let expected_blog_id = blog_id;
        let expected_url = url;
        let expected_github_id = github_id;
        let expected_github_name = github_name;

        assert_eq!(actual.blog_id, expected_blog_id);
        assert_eq!(actual.url, expected_url);
        assert_eq!(actual.github_id, expected_github_id);
        assert_eq!(actual.github_name, expected_github_name);
    }
}
