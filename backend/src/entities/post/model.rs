use rocket::serde::{Deserialize, Serialize};

use crate::entities::Profile;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub content: String,
}
