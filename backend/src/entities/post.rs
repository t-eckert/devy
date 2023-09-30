use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Option<String>,
    pub slug: String,

    pub blog_slug: String,
    pub blog_name: String,
    pub author_name: Option<String>,
    pub author_slug: String,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    pub title: String,
    pub content: String,
}

impl Post {
    pub async fn get_by_feed(
        pool: PgPool,
        feed_id: String,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, StatusCode> {
        match feed_id.as_str() {
            "new" => Self::get_by_feed_new(pool, limit, offset).await,
            _ => Err(StatusCode::NOT_FOUND),
        }
    }

    async fn get_by_feed_new(
        pool: PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Post>, StatusCode> {
        dbg!(&limit, &offset);

        sqlx::query_file_as!(Post, "queries/post_get_by_feed_new.sql", limit)
            .fetch_all(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}
