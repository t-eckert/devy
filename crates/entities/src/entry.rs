use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub post_slug: String,
    pub title: String,
    pub body: String,
    pub likes: Option<i64>,

    pub blog_slug: Option<String>,
    pub blog_name: Option<String>,

    pub author_slug: Option<String>,
    pub author_name: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
