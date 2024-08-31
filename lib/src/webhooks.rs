use crate::{
    db::DBConn,
    entities::{Webhook, WebhookType},
};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

pub fn determine_type(headers: HashMap<String, String>, payload: &Value) -> WebhookType {
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

pub struct ID {
    pub id: Uuid,
}

pub struct WebhookRepository;

impl WebhookRepository {
    pub async fn insert(
        db_conn: &DBConn,
        webhook_type: WebhookType,
        payload: Value,
    ) -> Result<Uuid, anyhow::Error> {
        Ok(sqlx::query_file_as!(
            ID,
            "queries/insert_webhook.sql",
            webhook_type.as_str(),
            payload
        )
        .fetch_one(db_conn)
        .await?
        .id)
    }

    pub async fn get(db_conn: &DBConn, id: Uuid) -> Result<Webhook, anyhow::Error> {
        Ok(
            sqlx::query_as!(Webhook, "SELECT id, type::TEXT as webhook_type, payload, to_char(received_at, 'YYYY-MM-DDThh:mm:ss.ss') AS received_at FROM webhook WHERE id = $1", id)
                .fetch_one(db_conn)
                .await?,
        )
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
