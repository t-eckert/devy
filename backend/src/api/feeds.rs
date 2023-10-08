use crate::{
    entities::{Feed, Post},
    store::Store,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use std::collections::HashMap;

pub async fn get_feed_by_id(Path(feed_id): Path<String>) -> Result<Json<Feed>, StatusCode> {
    match feed_id.as_str() {
        "new" => Ok(Json(Feed {
            id: "new".to_string(),
            name: "New".to_string(),
        })),
        "popular" => Ok(Json(Feed {
            id: "popular".to_string(),
            name: "Popular".to_string(),
        })),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_feed_posts_by_id(
    State(store): State<Store>,
    Path(feed_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Post>>, StatusCode> {
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
        Post::get_by_feed(&store.pool, feed_id, limit, offset)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}
