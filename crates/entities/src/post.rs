use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
