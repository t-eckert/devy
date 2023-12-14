use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::error::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Uuid,
    pub slug: String,

    pub blog_slug: Option<String>,
    pub blog_name: Option<String>,
    pub author_name: Option<String>,
    pub author_username: Option<String>,

    pub title: String,
    pub content: String,

    pub likes: Option<i64>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

pub struct NewPost {
    pub slug: String,
    pub blog_slug: String,
    pub title: String,
    pub content: String,
}

pub struct PostRepository {}

impl PostRepository {
    pub async fn insert(pool: &PgPool, post: NewPost) -> Result<()> {
        let _ = sqlx::query_file_as!(
            Post,
            "queries/post_insert.sql",
            post.slug,
            post.blog_slug,
            post.title,
            post.content,
        )
        .fetch_one(pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Post> {
        Ok(sqlx::query_file_as!(Post, "queries/post_get_by_id.sql", id)
            .fetch_one(pool)
            .await?)
    }

    pub async fn get_by_blog_slug(pool: &PgPool, blog_slug: &str) -> Result<Post> {
        Ok(
            sqlx::query_file_as!(Post, "queries/post_get_by_blog_slug.sql", blog_slug)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_by_blog_slug_and_post_slug(
        pool: &PgPool,
        blog_slug: &str,
        post_slug: &str,
    ) -> Result<Post> {
        Ok(sqlx::query_file_as!(
            Post,
            "queries/post_get_by_blog_slug_and_post_slug.sql",
            post_slug,
            blog_slug
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn get_by_feed(
        pool: &PgPool,
        feed_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>> {
        Ok(
            sqlx::query_file_as!(Post, "queries/post_get_by_feed.sql", limit, offset)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn get_by_feed_new(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Post>> {
        Ok(
            sqlx::query_file_as!(Post, "queries/post_get_by_feed_new.sql", limit, offset)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn get_by_feed_popular(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<Post>> {
        Ok(
            sqlx::query_file_as!(Post, "queries/post_get_by_feed_popular.sql", limit, offset)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Vec<Post>> {
        Ok(
            sqlx::query_file_as!(Post, "queries/post_get_by_username.sql", username)
                .fetch_all(pool)
                .await?,
        )
    }

    pub async fn get_liked_by_username(pool: &PgPool, username: &str) -> Result<Vec<Post>> {
        Ok(
            sqlx::query_file_as!(Post, "queries/post_get_liked_by_username.sql", username)
                .fetch_all(pool)
                .await?,
        )
    }
}
