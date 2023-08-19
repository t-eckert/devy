use rocket::serde::{Deserialize, Serialize};

use crate::entities::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub blog_slug: String,
    pub blog_name: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub author: Profile,
    pub created_at: String,
    pub updated_at: String,
    pub likes: u32,
}
