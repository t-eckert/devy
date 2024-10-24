use crate::posts::Post;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Changeset {
    pub blog_id: Uuid,
    pub blog_slug: String,
    pub dir: String,
    pub post_changeset: PostChangeset,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostChangeset {
    pub post_changes: Vec<PostChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PostChange {
    Addition(PostAddition),
    Modification(PostModification),
    Deletion(PostDeletion),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAddition {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostModification {
    pub path: String,
    pub from_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostDeletion {
    pub path: String,
}
