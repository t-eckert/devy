use rocket::serde::{Deserialize, Serialize};

use super::fixtures;
use crate::models::Post;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Feed {
    pub id: String,
    pub name: String,
    pub posts: Vec<Post>,
}

impl Feed {
    pub fn new(id: String, name: String, posts: Vec<Post>) -> Self {
        Feed { id, name, posts }
    }

    pub fn get_by_id(id: String) -> Option<Self> {
        match id.as_str() {
            "new" => Some(Feed::get_feed_new()),
            "popular" => Some(Feed::get_feed_popular()),
            _ => None,
        }
    }

    pub fn get_feed_new() -> Self {
        fixtures::feed_new()
    }

    pub fn get_feed_popular() -> Self {
        fixtures::feed_popular()
    }
}
