use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub id: Uuid,
    pub blog_id: Uuid,

    pub url: String,
    pub metadata: Value,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
