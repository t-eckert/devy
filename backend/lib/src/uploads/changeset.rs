use crate::posts::Post;
use serde::{Deserialize, Serialize};

use super::DevyConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Changeset {
    pub devy_config: DevyConfig,
    pub posts: PostChangeset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostChangeset {
    added: Vec<Post>,
    updated: Vec<Post>,
    deleted: Vec<Post>,
}
