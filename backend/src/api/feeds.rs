use super::error::Result;
use crate::{
    entities::{FeedConfig, FeedConfigRepository, Post, PostRepository},
    store::Store,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::collections::HashMap;
use uuid::{uuid, Uuid};

/// Get posts by feed id.
/// GET /feeds/:feed_id/posts
pub async fn get_posts_by_feed_id(
    State(store): State<Store>,
    Path(feed_id): Path<Uuid>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Post>>> {
    let limit = params
        .get("limit")
        .unwrap_or(&"30".to_string())
        .parse::<i64>()
        .unwrap_or(30);
    let offset = params
        .get("offset")
        .unwrap_or(&"0".to_string())
        .parse::<i64>()
        .unwrap_or(0);

    Ok(Json(
        PostRepository::get_by_feed(&store.pool, feed_id, limit, offset).await?,
    ))
}

/// Get a feed config by feed id.
/// GET /feeds/:feed_id/config
pub async fn get_feed_config_by_id(
    State(store): State<Store>,
    Path(id): Path<Uuid>,
) -> Result<Json<FeedConfig>> {
    Ok(Json(
        FeedConfigRepository::get_by_id(&store.pool, id).await?,
    ))
}

/// Get a feed config for the "new" feed.
/// GET /feeds/new/config
pub async fn get_feed_config_for_new() -> Result<Json<FeedConfig>> {
    Ok(Json(FeedConfig::new(
        uuid!("5941b29d-246d-4897-a69e-3201f6ad8715"),
        "New".to_string(),
    )))
}

/// Get a feed config for the "popular" feed.
/// GET /feeds/popular/config
pub async fn get_feed_config_for_popular() -> Result<Json<FeedConfig>> {
    Ok(Json(FeedConfig::new(
        uuid!("e9173695-1b31-465f-9e79-a80224be44ad"),
        "Popular".to_string(),
    )))
}
