use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
pub struct DevyConfig {
    root: String,
}
