use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Like {
    pub profile_id: i32,
    pub post_id: i32,
}
