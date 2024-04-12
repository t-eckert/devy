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

/// `GET /feeds/:feed_id/config`
///
/// Get the feed config by a feed id.
async fn get_id(State(store): State<Store>, Path(id): Path<String>) -> Result<Json<Feed>> {
    unimplemented!()
}

/// `GET /feeds/recent/config`
///
/// Get the feed config for the "recent" feed.
async fn get_recent_config() -> Result<Json<FeedConfig>> {
    Ok(Json(FeedConfig::new("recent", "Recent")))
}

/// `GET /feeds/popular/config`
///
/// Get the feed config for the "popular" feed.
async fn get_popular_feed() -> Result<Json<FeedConfig>> {
    Ok(Json(FeedConfig::new("popular", "Popular")))
}

/// `GET /feeds/:feed_id/config`
///
/// Get the feed config by a feed id.
async fn get_id_config(
    State(store): State<Store>,
    Path(id): Path<String>,
) -> Result<Json<FeedConfig>> {
    Ok(Json(
        feed_config::get_by_id(&store.db, str_to_uuid(id)?).await?,
    ))
}

fn str_to_uuid(s: String) -> Result<Uuid> {
    Uuid::parse_str(&s).map_err(|_| Error::StatusCode(StatusCode::BAD_REQUEST))
}
