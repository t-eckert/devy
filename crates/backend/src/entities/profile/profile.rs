use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Option<String>,
    pub user_id: Option<String>,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Profile {
    pub fn new(user_id: String, display_name: String, avatar_url: Option<String>) -> Self {
        Self {
            id: None,
            user_id: Some(user_id),
            display_name: Some(display_name),
            avatar_url,
            bio: None,
            website: None,
            created_at: None,
            updated_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileForUpsert {
    pub id: Option<String>,
    pub user_id: Option<String>,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl ProfileForUpsert {
    pub fn new(user_id: String, display_name: String, avatar_url: Option<String>) -> Self {
        Self {
            id: None,
            user_id: Some(user_id),
            display_name: Some(display_name),
            avatar_url,
            bio: None,
            website: None,
            created_at: None,
            updated_at: None,
        }
    }
}
