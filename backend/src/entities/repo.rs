use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    id: Option<String>,
    url: Option<String>,
    name: Option<String>,

    github_id: Option<i32>,
    github_name: Option<String>,
    metadata: Option<String>,

    created_at: Option<String>,
    updated_at: Option<String>,
}

impl Repo {}
