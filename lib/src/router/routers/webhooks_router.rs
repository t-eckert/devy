use crate::db::{upload, webhook};
use crate::entities::{Webhook, WebhookType};
use crate::router::error::Result;
use crate::store::Store;
use axum::Router;
use axum::{extract::State, routing::post, Json};
use http::{HeaderMap, StatusCode};
use serde_json::{to_string, Value};

pub struct WebhooksRouter;

impl WebhooksRouter {
    pub fn create(store: Store) -> Router<Store> {
        Router::new()
            .route("/webhooks", post(receive))
            .with_state(store)
    }
}

async fn receive(
    State(store): State<Store>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    dbg!(&headers);
    dbg!(&payload);

    // This will only vibe with GitHub headers for now, but that's future problem.
    let event = format!(
        "webhook.github.{}",
        headers
            .get("x-github-event")
            .map(|header_value| header_value.to_str().unwrap_or_default())
            .unwrap_or_default()
    );

    let webhook = match webhook::insert(&store.db, &event, payload).await {
        Ok(webhook) => webhook,
        Err(e) => return Err(e.into()),
    };

    match webhook.webhook_type {
        WebhookType::GitHubPushEvent => {
            let repo = webhook.payload["repository"]["clone_url"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            store
                .uploader
                .upload(&store.db, upload::insert(&store.db, None, repo).await?)
                .await?;
        }
        WebhookType::Generic => {}
    }

    Ok(StatusCode::OK)
}
