use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub name: String,
    pub slug: String,

    pub username: String,
    pub display_name: Option<String>,
    pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Blog {
    pub async fn upsert(self, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let _ = sqlx::query_file_as!(
            Self,
            "queries/blog_upsert.sql",
            self.name,
            self.slug,
            self.username,
            self.description
        )
        .fetch_one(pool)
        .await;

        Blog::get_by_slug(pool, self.slug).await
    }

    pub async fn get_by_slug(pool: &PgPool, slug: String) -> Result<Self, sqlx::Error> {
        sqlx::query_file_as!(Self, "queries/blog_get_by_slug.sql", slug)
            .fetch_one(pool)
            .await
    }

    pub async fn get_by_username(
        pool: &PgPool,
        username: String,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_file_as!(Self, "queries/blog_get_by_username.sql", username)
            .fetch_all(pool)
            .await
    }
}
