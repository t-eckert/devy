use super::error::Result;
use crate::{
    entities::{FeedConfig, FeedConfigRepository},
    store::Store,
};
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::{uuid, Uuid};

pub async fn get_by_id(
    State(store): State<Store>,
    Path(id): Path<Uuid>,
) -> Result<Json<FeedConfig>> {
    Ok(Json(
        FeedConfigRepository::get_by_id(&store.pool, id).await?,
    ))
}

pub async fn get_new() -> Result<Json<FeedConfig>> {
    Ok(Json(FeedConfig::new(
        uuid!("5941b29d-246d-4897-a69e-3201f6ad8715"),
        "New".to_string(),
    )))
}

pub async fn get_popular() -> Result<Json<FeedConfig>> {
    Ok(Json(FeedConfig::new(
        uuid!("e9173695-1b31-465f-9e79-a80224be44ad"),
        "Popular".to_string(),
    )))
}
