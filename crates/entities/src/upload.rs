use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub previous_upload_id: Option<Uuid>,

    pub status: String,
    pub repo: String,
    pub sha: String,
    pub logs: Option<Vec<String>>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
