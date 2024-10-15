use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,

    pub username: String,
    pub email: Option<String>,
    pub github_username: Option<String>,
    pub role: String,
    pub status: String,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
