use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::{uuid, Uuid};

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

pub struct UpsertPost {
    pub id: Option<Uuid>,
    pub slug: Option<String>,
    pub blog_slug: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl UpsertPost {
    pub fn new(slug: String, title: String) -> Self {
        Self {
            id: None,
            slug: Some(slug),
            blog_slug: None,
            title: Some(title),
            content: None,
        }
    }

    pub fn blog_slug(mut self, blog_slug: String) -> Self {
        self.blog_slug = Some(blog_slug);
        self
    }

    pub fn title_from_content(mut self) -> Self {
        let title = self
            .content
            .clone()
            .ok_or(Error::MissingField("content".to_string()))
            .and_then(|content| Self::markdown_title(&content))
            .unwrap_or("Untitled".to_string());

        self.title = Some(title);
        self
    }

    fn markdown_title(markdown: &str) -> Result<String> {
        let title_pattern = Regex::new(r"^#\s+(.+)").unwrap();

        for line in markdown.lines() {
            // Attempt to match the line against the title pattern
            if let Some(captures) = title_pattern.captures(line) {
                // Extract the captured title group and return it
                if let Some(title) = captures.get(1) {
                    return Ok(title.as_str().to_string());
                }
            }
        }

        Err(Error::MissingField("title".to_string()))
    }
}

pub struct PostRepository {}

impl PostRepository {
    pub async fn upsert(pool: &PgPool, post: UpsertPost) -> Result<Post> {
        // TODO
        Ok(Post {
            id: uuid!("5941b29d-246d-4897-a69e-3201f6ad8715"),
            slug: "slug".to_string(),
            blog_slug: Some("blog_slug".to_string()),
            blog_name: None,
            author_name: None,
            author_username: None,
            title: "title".to_string(),
            content: "content".to_string(),
            likes: None,
            created_at: None,
            updated_at: None,
        })
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Post> {
        Ok(sqlx::query_file_as!(Post, "queries/post_get_by_id.sql", id)
            .fetch_one(pool)
            .await?)
    }

    pub async fn get_by_blog_slug(pool: &PgPool, blog_slug: &str) -> Result<Vec<Post>> {
        Ok(
            sqlx::query_file_as!(Post, "queries/post_get_by_blog_slug.sql", blog_slug)
                .fetch_all(pool)
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
        let new = uuid!("5941b29d-246d-4897-a69e-3201f6ad8715");
        let popular = uuid!("e9173695-1b31-465f-9e79-a80224be44ad");

        match feed_id {
            id if id == new => Self::get_by_feed_new(pool, limit, offset).await,
            id if id == popular => Self::get_by_feed_popular(pool, limit, offset).await,
            _ => Err(Error::EntityNotFound),
        }
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
        return Ok(vec![]);

        // Ok(
        //     sqlx::query_file_as!(Post, "queries/post_get_liked_by_username.sql", username)
        //         .fetch_all(pool)
        //         .await?,
        // )
    }
}
