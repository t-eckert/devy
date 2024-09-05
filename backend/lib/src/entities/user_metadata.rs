use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMetadata {
    pub has_installed_devy: bool,
    pub num_blogs: i32,
    pub tags: Vec<String>,
}
