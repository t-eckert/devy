use crate::error::Result;
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json,
};
use db::{feed_config, post};
use entities::{FeedConfig, Post};
use std::collections::HashMap;
use store::Store;
use uuid::{uuid, Uuid};

pub struct FeedsRouter;

/// /feeds routes
impl FeedsRouter {
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/feeds/new/posts", get(get_posts_for_new))
            .route("/feeds/popular/posts", get(get_posts_for_popular))
            .route("/feeds/:feed_id/posts", get(get_posts_by_feed_id))
            .route("/feeds/new/config", get(get_feed_config_for_new))
            .route("/feeds/popular/config", get(get_feed_config_for_popular))
            .route("/feeds/:feed_id/config", get(get_feed_config_by_id))
            .with_state(store)
    }
}

/// GET /feeds/:feed_id/posts
///
/// Get posts by feed id.
async fn get_posts_by_feed_id(
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

    dbg!(&limit);
    dbg!(&offset);

    Ok(Json(
        post::get_by_feed(&store.db, feed_id, limit, offset).await?,
    ))
}

/// GET /feeds/new/posts
///
/// Get posts for the "new" feed.
async fn get_posts_for_new(
    State(store): State<Store>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Post>>> {
    let feed_id = uuid!("5941b29d-246d-4897-a69e-3201f6ad8715");
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
        post::get_by_feed(&store.db, feed_id, limit, offset).await?,
    ))
}

/// GET /feeds/popular/posts
///
/// Get posts for the "popular" feed.
async fn get_posts_for_popular(
    State(store): State<Store>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Post>>> {
    let feed_id = uuid!("e9173695-1b31-465f-9e79-a80224be44ad");
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
        post::get_by_feed(&store.db, feed_id, limit, offset).await?,
    ))
}

/// Get the feed config by a feed id.
/// GET /feeds/:feed_id/config
async fn get_feed_config_by_id(
    State(store): State<Store>,
    Path(id): Path<Uuid>,
) -> Result<Json<FeedConfig>> {
    Ok(Json(feed_config::get_by_id(&store.db, id).await?))
}

/// Get the feed config for the "new" feed.
/// GET /feeds/new/config
async fn get_feed_config_for_new() -> Result<Json<FeedConfig>> {
    Ok(Json(FeedConfig::new(
        uuid!("5941b29d-246d-4897-a69e-3201f6ad8715"),
        "New".to_string(),
    )))
}

/// Get the feed config for the "popular" feed.
/// GET /feeds/popular/config
async fn get_feed_config_for_popular() -> Result<Json<FeedConfig>> {
    Ok(Json(FeedConfig::new(
        uuid!("e9173695-1b31-465f-9e79-a80224be44ad"),
        "Popular".to_string(),
    )))
}
