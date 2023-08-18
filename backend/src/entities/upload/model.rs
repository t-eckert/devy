use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::db::DB;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Upload {
    pub id: String,
}
