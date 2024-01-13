use super::error::Result;
use crate::{
    entities::{like, Like},
    store::Store,
};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    Json,
};
use uuid::Uuid;

pub async fn get_ids_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Like>>> {
    Ok(Json(like::get_by_username(&store.pool, username).await?))
}

pub async fn post_like(
    State(store): State<Store>,
    ExtractJson(like): ExtractJson<like::LikeForUpsert>,
) -> Result<Json<Like>> {
    Ok(Json(like::upsert(&store.pool, like).await?))
}

pub async fn delete_like(
    State(store): State<Store>,
    Path((post_id, profile_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    Ok(like::delete(&store.pool, profile_id, post_id).await?)
}
