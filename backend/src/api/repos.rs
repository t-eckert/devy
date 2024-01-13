use super::*;
use crate::entities::{repo, Repo};
use crate::store::Store;
use axum::{
    extract::{Json as ExtractJson, State},
    Json,
};

pub async fn insert(
    State(store): State<Store>,
    ExtractJson(repo): ExtractJson<repo::RepoForUpsert>,
) -> Result<Json<Repo>> {
    Ok(Json(repo::insert(&store.pool, repo).await?))
}
