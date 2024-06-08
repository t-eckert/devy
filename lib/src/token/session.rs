use crate::entities::{Profile, User};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {}

impl Session {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<Value> for Session {
    fn from(value: Value) -> Self {
        Self {}
    }
}
