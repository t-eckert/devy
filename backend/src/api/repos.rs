use super::*;
use crate::entities::{repo, Repo};
use crate::store::Store;
use axum::{
    extract::{Json as ExtractJson, State},
    Json,
};

///
pub async fn upsert(
    State(store): State<Store>,
    ExtractJson(repo): ExtractJson<repo::RepoForUpsert>,
) -> Result<Json<Repo>> {
    Ok(Json(repo::upsert(&store.pool, repo).await?))
}
