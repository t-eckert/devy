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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoForUpsert {
    pub blog_id: Uuid,
    pub url: String,
    pub github_id: i64,
    pub github_name: String,
}

impl RepoForUpsert {
    pub fn new(blog_id: Uuid, url: String, github_id: i64, github_name: String) -> Self {
        Self {
            blog_id,
            url,
            github_id,
            github_name,
        }
    }
}
