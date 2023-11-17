use super::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

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
