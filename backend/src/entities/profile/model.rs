use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct Profile {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Profile {
    pub fn new(display_name: String, avatar_url: Option<String>) -> Self {
        Self {
            display_name: Some(display_name),
            avatar_url,
            created_at: None,
            updated_at: None,
        }
    }
}
