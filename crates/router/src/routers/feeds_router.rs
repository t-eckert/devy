use std::collections::HashMap;

use crate::{error::Result, Error};
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json,
};
use db::{feed, feed_config};
use entities::{Feed, FeedConfig};
use http::StatusCode;
use store::Store;
use uuid::Uuid;

pub struct FeedsRouter;

/// `/feeds` routes
///
/// These routes get and manage feeds of entities.
impl FeedsRouter {
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/feeds/recent", get(get_recent))
            .with_state(store)
    }
}

/// `GET /feeds/recent`
///
/// Get a feed of the most recent posts.
async fn get_recent(State(store): State<Store>) -> Result<Json<Feed>> {
    Ok(Json(feed::get_recent(&store.db).await?))
}

/// `GET /feeds/popular`
///
/// Get a feed of the most popular posts.
async fn get_popular(
    State(store): State<Store>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Feed>> {
    unimplemented!()
}
