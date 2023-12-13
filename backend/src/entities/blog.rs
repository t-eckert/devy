use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

use super::error::Result;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBlog {
    pub name: String,
    pub slug: String,

    pub username: String,
    pub display_name: Option<String>,
    pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Blog {
    pub async fn get_by_id(pool: &PgPool, id: String) -> Result<Self> {
        let uuid = Uuid::parse_str(&id).unwrap();

        Ok(
            sqlx::query_file_as!(Self, "queries/blog_get_by_id.sql", uuid)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_by_slug(pool: &PgPool, slug: String) -> Result<Self> {
        Ok(
            sqlx::query_file_as!(Self, "queries/blog_get_by_slug.sql", slug)
                .fetch_one(pool)
                .await?,
        )
    }
}

pub struct BlogRepository {}

impl BlogRepository {
    pub async fn upsert(pool: &PgPool, blog: NewBlog) -> Result<Blog> {
        let _ = sqlx::query_file_as!(
            Blog,
            "queries/blog_upsert.sql",
            blog.name,
            blog.slug,
            blog.username,
            blog.description,
        )
        .fetch_one(pool)
        .await?;

        Blog::get_by_slug(pool, blog.slug).await
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Blog> {
        Ok(sqlx::query_file_as!(Blog, "queries/blog_get_by_id.sql", id)
            .fetch_one(pool)
            .await?)
    }

    pub async fn get_by_slug(pool: &PgPool, slug: String) -> Result<Blog> {
        Ok(
            sqlx::query_file_as!(Blog, "queries/blog_get_by_slug.sql", slug)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_by_username(pool: &PgPool, username: String) -> Result<Vec<Blog>> {
        Ok(
            sqlx::query_file_as!(Blog, "queries/blog_get_by_username.sql", username)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn delete_by_slug(pool: &PgPool, slug: String) -> Result<()> {
        sqlx::query_file!("queries/blog_delete_by_slug.sql", slug)
            .execute(pool)
            .await?;

        Ok(())
    }
}
