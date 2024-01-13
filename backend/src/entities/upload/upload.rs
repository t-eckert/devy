use super::*;
use crate::entities::webhook::{Webhook, WebhookType};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub previous_upload_id: Option<Uuid>,

    pub status: String,
    pub repo: String,
    pub logs: Option<Vec<String>>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadForUpsert {
    pub previous_upload_id: Option<Uuid>,
    pub repo: String,
}

impl TryFrom<Webhook> for UploadForUpsert {
    type Error = crate::entities::error::Error;
    fn try_from(value: Webhook) -> std::result::Result<Self, Self::Error> {
        match value.webhook_type {
            WebhookType::GitHubPushEvent => Ok(Self {
                previous_upload_id: None,
                repo: value.payload["repository"]["url"].to_string(),
            }),

            _ => Err(crate::entities::error::Error::Malformed(
                "Webhook does not have webhook_type of `webhook.github.push`".to_string(),
            )),
        }
    }
}
