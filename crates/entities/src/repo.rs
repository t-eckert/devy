use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub id: Uuid,
    pub url: String,
    pub blog_id: Uuid,

    pub github_id: i64,
    pub github_name: String,
    pub metadata: Value,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
