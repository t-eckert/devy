use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;

use crate::entities::Post;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Feed {
    pub id: String,
    pub name: String,
    pub posts: Vec<Post>,
}
