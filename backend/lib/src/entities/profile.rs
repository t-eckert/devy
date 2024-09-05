use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
