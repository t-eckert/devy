use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub metadata: Option<Value>,
}

impl RepoForUpsert {
    pub fn new(blog_id: Uuid, url: String, github_id: i64, github_name: String) -> Self {
        Self {
            blog_id,
            url,
            github_id,
            github_name,
            metadata: None,
        }
    }

    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}
