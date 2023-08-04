use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use super::fixtures;
use crate::db::DB;
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

    pub fn get_by_id(db: Connection<DB>, id: String) -> Option<Self> {
        match id.as_str() {
            "new" => Some(Feed::get_feed_new(db)),
            "popular" => Some(Feed::get_feed_popular(db)),
            _ => None,
        }
    }

    pub fn get_feed_new(_db: Connection<DB>) -> Self {
        fixtures::feed_new()
    }

    pub fn get_feed_popular(_db: Connection<DB>) -> Self {
        fixtures::feed_popular()
    }

    pub fn upsert(&self, _db: Connection<DB>) -> Self {
        Feed::new(self.id.clone(), self.name.clone(), self.posts.clone())
    }
}
