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
    pub metadata: Option<Value>,

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
    pub metadata: Value,
}
