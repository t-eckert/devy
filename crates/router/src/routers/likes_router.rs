use crate::error::Result;
use axum::{
    extract::{Json as ExtractJson, Path, State},
    routing::{delete, get, post},
    Json,
};
use db::like;
use entities::Like;
use store::Store;
use uuid::Uuid;

pub struct LikesRouter;

impl LikesRouter {
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/likes", post(post_like))
            .route("/likes/:username", get(get_by_username))
            .route("/likes/:profile_id/:post_id", delete(delete_like))
            .with_state(store)
    }
}

async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Like>>> {
    Ok(Json(like::get_by_username(&store.db, username).await?))
}

async fn post_like(
    State(store): State<Store>,
    ExtractJson(like): ExtractJson<Like>,
) -> Result<Json<Like>> {
    Ok(Json(
        like::upsert(&store.db, like.profile_id, like.post_id).await?,
    ))
}

async fn delete_like(
    State(store): State<Store>,
    Path((profile_id, post_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    Ok(like::delete(&store.db, profile_id, post_id).await?)
}
