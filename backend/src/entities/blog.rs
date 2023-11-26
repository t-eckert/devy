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
pub struct BlogInput {
    pub name: String,
    pub slug: String,

    pub username: String,
    pub display_name: Option<String>,
    pub description: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Blog {
    pub async fn upsert(self, pool: &PgPool) -> Result<Self> {
        let _ = sqlx::query_file_as!(
            Self,
            "queries/blog_upsert.sql",
            self.name,
            self.slug,
            self.username,
            self.description,
        )
        .fetch_one(pool)
        .await?;

        Blog::get_by_slug(pool, self.slug).await
    }

    pub async fn get_by_id(pool: &PgPool, id: String) -> Result<Self> {
        let uuid = Uuid::parse_str(&id).unwrap();

        Ok(sqlx::query_as!(
            Self,
            r#"
                SELECT 
                    name, slug,
                    to_char(blog.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                    to_char(blog.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
                    username, display_name, description
                FROM "blog" LEFT JOIN (
                    SELECT 
                        profile.id, display_name, username
                    FROM "profile" LEFT JOIN "user"
                    ON user_id="user".id
                ) AS "profile" ON profile_id="profile".id
                WHERE blog.id=$1;
                "#,
            uuid
        )
        .fetch_one(pool)
        .await?)
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
    pub async fn upsert(pool: &PgPool, blog: BlogInput) -> Result<Blog> {
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
        Ok(sqlx::query_as!(
            Blog,
            r#"
                SELECT 
                    name, slug,
                    to_char(blog.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                    to_char(blog.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
                    username, display_name, description
                FROM "blog" LEFT JOIN (
                    SELECT 
                        profile.id, display_name, username
                    FROM "profile" LEFT JOIN "user"
                    ON user_id="user".id
                ) AS "profile" ON profile_id="profile".id
                WHERE blog.id=$1;
                "#,
            id
        )
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
