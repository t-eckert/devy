use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Uuid,
    pub blog_id: Uuid,

    pub title: String,
    pub slug: String,
    pub body: String,

    pub likes: Option<i64>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
