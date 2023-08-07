use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::db::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Upload {
    pub id: String,
}

impl Upload {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn upsert(&mut self, db: Connection<DB>) {
        println!("upserting upload: {:?}", self);
    }

    pub fn get_by_id(db: Connection<DB>, id: String) -> Option<Self> {
        Some(Self::new(id))
    }
}
