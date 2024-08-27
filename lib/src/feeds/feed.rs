use super::Entry;
use super::FeedConfig;
use serde::{Deserialize, Serialize};

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub feed_config: FeedConfig,
    pub entries: Vec<Entry>,
}
