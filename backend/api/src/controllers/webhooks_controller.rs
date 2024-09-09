use super::Result;
use crate::store::Store;
use lib::db::upload;
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
        let webhook_id = webhooks::WebhookRepository::insert(&store.db_conn, webhook_type, payload)
            .await
            .map_err(|_| super::Error::Generic("failed to insert".to_string()))?;
        let webhook = webhooks::WebhookRepository::get(&store.db_conn, webhook_id)
            .await
            .map_err(|_| super::Error::Generic("failed to get".to_string()))?;

        match webhook.webhook_type {
            webhooks::WebhookType::GitHubPushEvent => {
                let repo = webhook.payload["repository"]["clone_url"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                let _ = store
                    .uploader
                    .upload(
                        &store.db_conn,
                        upload::insert(&store.db_conn, None, repo).await?,
                    )
                    .await;
            }
            webhooks::WebhookType::Uncategorized => {}
        }

        Ok(())
    }
}
