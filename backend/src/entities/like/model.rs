use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Like {
    pub profile_id: Option<String>,
    pub post_id: Option<String>,
}
