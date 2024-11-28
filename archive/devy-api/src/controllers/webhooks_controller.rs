use super::Result;
use crate::store::Store;
use lib::repositories::{BlogRepository, UploadRepository, WebhookRepository};
use lib::webhooks;
use serde_json::Value;
use std::collections::HashMap;

pub struct WebhooksController;

impl WebhooksController {
    pub async fn handle(
        store: Store,
        headers: HashMap<String, String>,
        payload: Value,
    ) -> Result<()> {
        let webhook_type = webhooks::determine_type(headers, &payload);
        let webhook_id = WebhookRepository::insert(&store.db_conn, webhook_type, payload)
            .await
            .map_err(|_| super::Error::Generic("failed to insert".to_string()))?;
        let webhook = WebhookRepository::get(&store.db_conn, webhook_id)
            .await
            .map_err(|_| super::Error::Generic("failed to get".to_string()))?;

        match webhook.webhook_type {
            webhooks::WebhookType::GitHubPushEvent => {
                let repo = webhook.payload["repository"]["clone_url"]
                    .as_str()
                    .unwrap_or_default();
                let blog = BlogRepository::get_by_repo(&store.db_conn, repo)
                    .await?
                    .ok_or(super::Error::Generic("not found".to_string()))?;
                UploadRepository::insert(&store.db_conn, None, blog.id).await?;
            }
            webhooks::WebhookType::Uncategorized => {}
        }

        Ok(())
    }
}
