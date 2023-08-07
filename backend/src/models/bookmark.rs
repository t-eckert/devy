use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use super::fixtures;
use crate::db::DB;
use crate::models::Post;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Bookmark {
    id: Option<String>,
    pub user_id: String,
    pub post_id: String,
}

impl Bookmark {
    pub fn new(user_id: String, post_id: String) -> Self {
        Self {
            id: None,
            user_id,
            post_id,
        }
    }

    pub fn upsert(&mut self, db: Connection<DB>) {
        self.id = Some("0c60d9ba-635b-4b33-b0f6-5d8f5cc639f5".to_string());
        println!("upserting like: {:?}", self);
    }

    pub fn delete(&mut self, db: Connection<DB>) {
        println!("delete like: {:?}", self);
    }

    pub fn get_by_user_id(db: Connection<DB>, user_id: String) -> Option<Vec<Self>> {
        Some(vec![Self::new(
            user_id,
            "0c60d9ba-635b-4b33-b0f6-5d8f5cc639f5".to_string(),
        )])
    }
}
