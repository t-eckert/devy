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
    pub async fn get_by_feed(pool: PgPool, feed_id: String) -> Result<Vec<Post>, StatusCode> {
        Err(StatusCode::NOT_FOUND)
    }
}
