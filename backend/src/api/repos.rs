use super::error::Result;

use crate::{
    entities::{Repo, RepoInput, RepoRepository},
    store::Store,
};
use axum::{
    extract::{Json as ExtractJson, State},
    Json,
};

pub async fn insert(
    State(store): State<Store>,
    ExtractJson(repo): ExtractJson<RepoInput>,
) -> Result<Json<Repo>> {
    Ok(Json(RepoRepository::insert(&store.pool, repo).await?))
}
