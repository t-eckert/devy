use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::db::DB;

use super::Profile;

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

    pub fn get_by_id(db: Connection<DB>, id: &str) -> Option<Self> {
        None
    }

    pub fn get_by_blog_and_post_slug(
        db: Connection<DB>,
        blog_slug: String,
        pst_slug: String,
    ) -> Option<Self> {
        None
    }
}
