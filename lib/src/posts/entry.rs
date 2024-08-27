use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub id: Uuid,

    pub blog_slug: String,
    pub post_slug: String,

    pub title: String,
    pub body: String,
    pub likes: Option<i64>,

    pub blog_name: Option<String>,

    pub author_slug: Option<String>,
    pub author_name: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
