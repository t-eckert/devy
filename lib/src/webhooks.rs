use http::{HeaderMap, HeaderValue};
use crate::entities::{Webhook, WebhookType};
use serde_json::Value;

pub fn determine_type(headers: HeaderMap, payload: &Value) -> WebhookType {
    // GitHub will send the type of event in the header.
    let github_event = headers.get("x-github-event");
    if let Some(event) = github_event {
        match event.to_str().unwrap_or_default(){
            "push" => return WebhookType::GitHubPushEvent,
            _ => return WebhookType::Uncategorized,
        }
    }

    WebhookType::Uncategorized
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_type() {
        let mut headers = HeaderMap::new();
        headers.insert("x-github-event", HeaderValue::from_static("push"));

        let payload = serde_json::json!({});

        assert_eq!(determine_type(headers, &payload), WebhookType::GitHubPushEvent);
    }
}
