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

impl Upload {
    pub fn new(repo: &str, sha: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            previous_upload_id: None,
            status: "pending".to_string(),
            repo: repo.to_string(),
            sha: sha.to_string(),
            logs: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn dir(&self) -> String {
        format!("/tmp/{}", self.id)
    }
}
