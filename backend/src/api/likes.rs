use super::error::Result;
use crate::{
    entities::{Like, LikeRepository, NewLike},
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
    Ok(Json(
        LikeRepository::get_by_username(&store.pool, username).await?,
    ))
}

pub async fn post_like(
    State(store): State<Store>,
    ExtractJson(like): ExtractJson<NewLike>,
) -> Result<Json<Like>> {
    Ok(Json(LikeRepository::upsert(&store.pool, like).await?))
}

pub async fn delete_like(
    State(store): State<Store>,
    Path((post_id, profile_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    Ok(LikeRepository::delete(&store.pool, profile_id, post_id).await?)
}
