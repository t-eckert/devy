use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Like {
    id: Option<String>,
    pub user_id: String,
    pub post_id: String,
}
