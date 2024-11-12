use crate::posts::Entry;
use serde::{Deserialize, Serialize};

/// A feed is a continuously updating set of entries grouped by some common property.
/// As an example, the "popular" feed might contain the most liked posts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub config: FeedConfig,
    pub page: u32,
    pub page_size: i64,
    pub count: usize,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeedConfig {
    pub id: String,
    pub name: String,
}

impl FeedConfig {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}
