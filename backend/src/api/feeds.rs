use super::error::Result;
use crate::{
    entities::{Post, PostRepository},
    store::Store,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_feed_posts_by_id(
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
