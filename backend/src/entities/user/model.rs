use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub profile_id: Option<i32>,
    pub username: String,
    pub email: Option<String>,
    pub github_username: Option<String>,
}
