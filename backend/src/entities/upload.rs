use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Option<String>,

    pub previous_upload_id: Option<String>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    pub status: Option<String>,
    pub repo: Option<String>,
    pub logs: Option<Vec<String>>,
}
