use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Webhook {
    pub id: String,
    pub origin: String,
    pub created_at: String,
    pub body: String,
}
