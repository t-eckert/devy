use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: Uuid,

    pub webhook_type: WebhookType,
    pub payload: Value,

    pub received_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookType {
    GitHubPushEvent,
    Uncategorized,
}

impl WebhookType {
    pub fn as_str(&self) -> &str {
        match self {
            WebhookType::GitHubPushEvent => "webhook.github.push",
            WebhookType::Uncategorized => "webhook.uncategorized",
        }
    }
}

impl From<String> for WebhookType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "webhook.github.push" => WebhookType::GitHubPushEvent,
            _ => WebhookType::Uncategorized,
        }
    }
}

impl fmt::Display for WebhookType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebhookType::Uncategorized => write!(f, "webhook.uncategorized"),
            WebhookType::GitHubPushEvent => write!(f, "webhook.github.push"),
        }
    }
}
