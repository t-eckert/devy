use crate::posts::Entry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub config: CollectionConfig,
    pub page: u32,
    pub page_size: i64,
    pub count: usize,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CollectionConfig {
    pub id: String,
    pub name: String,
}

impl CollectionConfig {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}
