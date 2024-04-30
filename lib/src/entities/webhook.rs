use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: Option<String>,

    pub webhook_type: WebhookType,
    pub payload: Value,

    pub received_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookType {
    GitHubPushEvent,
    Generic,
}

impl From<String> for WebhookType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "webhook.github.push" => WebhookType::GitHubPushEvent,
            _ => WebhookType::Generic,
        }
    }
}

impl fmt::Display for WebhookType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WebhookType::Generic => write!(f, "webhook.generic"),
            WebhookType::GitHubPushEvent => write!(f, "webhook.github.push"),
        }
    }
}
