use super::error::Result;
use crate::{
    entities::{FeedConfig, FeedConfigRepository},
    store::Store,
};
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

pub async fn get_by_id(
    State(store): State<Store>,
    Path(id): Path<Uuid>,
) -> Result<Json<FeedConfig>> {
    Ok(Json(
        FeedConfigRepository::get_by_id(&store.pool, id).await?,
    ))
}
