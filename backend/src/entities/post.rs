use super::error::EntityError;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Option<String>,
    pub slug: Option<String>,

    pub blog_slug: Option<String>,
    pub blog_name: Option<String>,
    pub author_name: Option<String>,
    pub author_slug: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    pub title: Option<String>,
    pub content: Option<String>,
    pub likes: Option<i64>,
}

impl Post {
    pub async fn get_by_id(pool: &PgPool, id: String) -> Result<Self, EntityError> {
        sqlx::query_file_as!(
            Self,
            "queries/post_get_by_id.sql",
            Uuid::try_parse(id.as_str()).map_err(|_| EntityError::malformed("id is not uuid"))?
        )
        .fetch_one(pool)
        .await
        .map_err(|x| EntityError::Sqlx(x))
    }

    pub async fn get_by_blog_slug(
        pool: &PgPool,
        blog_slug: String,
    ) -> Result<Vec<Self>, EntityError> {
        Ok(
            sqlx::query_file_as!(Self, "queries/post_get_by_blog_slug.sql", blog_slug)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn get_by_blog_and_post_slug(
        pool: &PgPool,
        blog: String,
        post: String,
    ) -> Result<Self, EntityError> {
        Ok(sqlx::query_file_as!(
            Self,
            "queries/post_get_by_blog_slug_and_post_slug.sql",
            blog,
            post
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_by_feed(
        pool: &PgPool,
        feed_id: String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, anyhow::Error> {
        match feed_id.as_str() {
            "new" => Self::get_by_feed_new(pool, limit, offset)
                .await
                .map_err(|_| anyhow!("Cannot find feed")),
            "popular" => Self::get_by_feed_popular(pool, limit, offset)
                .await
                .map_err(|_| anyhow!("Cannot find feed")),
            _ => Err(anyhow!("Cannot find feed")),
        }
    }

    pub async fn get_by_username(
        pool: &PgPool,
        username: String,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_file_as!(Self, "queries/post_get_by_username.sql", username)
            .fetch_all(pool)
            .await
    }
    pub async fn get_liked_by_username(
        pool: &PgPool,
        username: String,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_file_as!(Self, "queries/post_get_liked_by_username.sql", username)
            .fetch_all(pool)
            .await
    }

    async fn get_by_feed_new(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_file_as!(Self, "queries/post_get_by_feed_new.sql", limit, offset)
            .fetch_all(pool)
            .await
    }

    async fn get_by_feed_popular(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_file_as!(Self, "queries/post_get_by_feed_popular.sql", limit, offset)
            .fetch_all(pool)
            .await
    }
}
