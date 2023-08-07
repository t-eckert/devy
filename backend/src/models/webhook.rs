use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use super::fixtures;
use crate::db::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Webhook {
    id: Option<String>,
    pub origin: String,
    pub created_at: String,
    pub body: String,
}

impl Webhook {}
