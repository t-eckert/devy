use rocket::serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: String,
    pub slug: String,
    pub blog: String,
    pub author: User,
    pub title: String,
    pub markdown: String,
}

impl Post {
    pub fn new(
        id: String,
        slug: String,
        blog: String,
        author: User,
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
}
