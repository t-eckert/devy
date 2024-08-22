use crate::router::error::Result;
use axum::Router;
use axum::{extract::State, routing::post, Json};
use http::{HeaderMap, StatusCode};

use lib::store::Store;
use lib::{
    db::{upload, webhook},
    entities::WebhookType,
    webhooks,
};
use serde_json::Value;

pub fn router(store: Store) -> Router<Store> {
    Router::new()
        .route("/webhooks", post(receive))
        .with_state(store)
}

async fn receive(
    State(store): State<Store>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> Result<StatusCode> {
    dbg!(&headers);
    dbg!(&payload);

    let webhook_type = webhooks::determine_type(headers, &payload);
    let webhook = match webhook::insert(&store.db_conn, webhook_type, payload).await {
        Ok(webhook) => webhook,
        Err(e) => return Err(e.into()),
    };

    match webhook.webhook_type {
        WebhookType::GitHubPushEvent => {
            let repo = webhook.payload["repository"]["clone_url"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            let _ = store
                .uploader
                .upload(&store.db_conn, upload::insert(&store.db_conn, None, repo).await?)
                .await;
        }
        WebhookType::Uncategorized => {}
    }

    Ok(StatusCode::ACCEPTED)
}
