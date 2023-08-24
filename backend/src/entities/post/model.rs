use rocket::serde::{Deserialize, Serialize};

use crate::entities::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
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
