use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use super::fixtures;
use crate::db::DB;
use crate::models::Post;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Like {
    pub user_id: String,
    pub post_id: String,
}

impl Like {
    pub fn new(user_id: String, post_id: String) -> Self {
        Self { user_id, post_id }
    }

    pub fn upsert(&self, db: Connection<DB>) -> Option<Self> {
        println!("upserting like: {:?}", self);
        None
    }

    pub fn delete(&self, db: Connection<DB>) -> Option<Self> {
        println!("delete like: {:?}", self);
        None
    }
}
