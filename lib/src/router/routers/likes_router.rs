use crate::{
    db::like,
    entities::Like,
    router::{error::Result, middleware::auth},
    store::Store,
};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    middleware,
    routing::{delete, get, post},
    Json,
};
use uuid::Uuid;

pub struct LikesRouter;

/// `/likes` routes
impl LikesRouter {
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/likes", post(post_like))
            .route("/likes/:username", get(get_by_username))
            .route("/likes/:profile_id/:post_id", delete(delete_like))
            .layer(middleware::from_fn_with_state(store.clone(), auth))
            .with_state(store)
    }
}

/// `GET /likes/:username`
async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Like>>> {
    Ok(Json(like::get_by_username(&store.db, username).await?))
}

/// `POST /likes`
async fn post_like(
    State(store): State<Store>,
    ExtractJson(like): ExtractJson<Like>,
) -> Result<Json<Like>> {
    Ok(Json(
        like::upsert(&store.db, like.profile_id, like.post_id).await?,
    ))
}

/// `DELETE /likes/:profile_id/:post_id`
async fn delete_like(
    State(store): State<Store>,
    Path((profile_id, post_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    Ok(like::delete(&store.db, profile_id, post_id).await?)
}
