use rocket::serde::{Deserialize, Serialize};

use crate::entities::Post;
use crate::entities::Profile;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Blog {
    pub name: String,
    pub slug: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub username: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
}
