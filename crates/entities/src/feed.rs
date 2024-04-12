use crate::Entry;
use crate::FeedConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub feed_config: FeedConfig,
    pub entries: Vec<Entry>,
}
