use super::Entry;
use super::FeedConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub feed_config: FeedConfig,
    pub entries: Vec<Entry>,
}
