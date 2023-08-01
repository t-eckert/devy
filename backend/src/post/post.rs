use rocket::serde::{Deserialize, Serialize};

use crate::profile::Profile;

use super::fixtures;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: String,
    pub slug: String,
    pub blog: String,
    pub author: Profile,
    pub title: String,
    pub markdown: String,
}

impl Post {
    pub fn new(
        id: String,
        slug: String,
        blog: String,
        author: Profile,
        title: String,
        markdown: String,
    ) -> Self {
        Self {
            id,
            slug,
            blog,
            author,
            title,
            markdown,
        }
    }

    pub fn get_by_id(id: &str) -> Option<Self> {
        Some(fixtures::post_map().get(id)?.clone())
    }
}
