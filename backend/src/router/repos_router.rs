use super::error::Result;
use crate::entities::{repo, Repo};
use crate::store::Store;
use axum::{
    extract::{Json as ExtractJson, State},
    Json,
};

pub fn make_router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/repos", axum::routing::post(upsert))
        .with_state(store)
}

///
pub async fn upsert(
    State(store): State<Store>,
    ExtractJson(repo): ExtractJson<repo::RepoForUpsert>,
) -> Result<Json<Repo>> {
    Ok(Json(repo::upsert(&store.pool, repo).await?))
}
