use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

pub fn determine_type(headers: HashMap<String, String>, _payload: &Value) -> WebhookType {
    // GitHub will send the type of event in the header.
    let github_event = headers.get("x-github-event");
    if let Some(event) = github_event {
        match event.as_str() {
            "push" => return WebhookType::GitHubPushEvent,
            _ => return WebhookType::Uncategorized,
        }
    }

    WebhookType::Uncategorized
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: Uuid,

    pub webhook_type: WebhookType,
    pub payload: Value,

    pub received_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_type() {
        let mut headers = HashMap::new();
        headers.insert("x-github-event".to_string(), "push".to_string());

        let payload = serde_json::json!({});

        assert_eq!(
            determine_type(headers, &payload),
            WebhookType::GitHubPushEvent
        );
    }
}
