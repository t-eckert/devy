use rocket::serde::{Deserialize, Serialize};

use crate::entities::Post;
use crate::entities::Profile;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Blog {
    pub name: String,
    pub slug: String,
    pub profile: Profile,
    pub posts: Vec<Post>,
}
