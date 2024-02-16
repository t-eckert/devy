use serde::{Deserialize, Serialize};
use serde_json::Value;
use database::Database;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Webhook {
    pub id: Option<String>,

    pub webhook_type: WebhookType,
    pub payload: Value,

    pub received_at: Option<String>,
}

impl Webhook {
    pub fn new(webhook_type: String, payload: Value) -> Self {
        Self {
            id: None,
            webhook_type: WebhookType::from(webhook_type),
            payload,
            received_at: None,
        }
    }

    pub async fn insert(self, pool: &PgPool) -> Result<Self> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            INSERT INTO "webhook" (type, payload)
            VALUES ($1, $2)
            RETURNING 
                id::TEXT,
                type::TEXT as webhook_type,
                payload,
                to_char(received_at, 'YYYY-MM-DDThh:mm:ss.ss') AS received_at;
            "#,
            self.webhook_type.to_string(),
            self.payload
        )
        .fetch_one(pool)
        .await?)
    }
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
