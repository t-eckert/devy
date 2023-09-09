use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Profile {
    pub display_name: Option<String>,
    // pub username: String,
    pub avatar_url: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
