use crate::router::{error::Result, middleware::auth};
use crate::store::Store;
use axum::{
    extract::{Json as ExtractJson, Path, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use lib::{db::like, entities::Like, token::Session};
use uuid::Uuid;

/// Create a new router for likes.
pub fn router(store: Store) -> Router<Store> {
    let open = Router::new()
        .route("/likes/:username", get(get_by_username))
        .with_state(store.clone());

    let protected = Router::new()
        .route("/likes", post(post_like))
        .route("/likes/:profile_id/:post_id", delete(delete_like))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store);

    Router::new().merge(open).merge(protected)
}

// GET /likes/:username
async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Like>>> {
    Ok(Json(like::get_by_username(&store.db_conn, username).await?))
}

// POST /likes
async fn post_like(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    ExtractJson(like): ExtractJson<Like>,
) -> Result<Json<Like>> {
    if session.profile_id != like.profile_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    Ok(Json(
        like::upsert(&store.db_conn, like.profile_id, like.post_id).await?,
    ))
}

// DELETE /likes/:profile_id/:post_id
async fn delete_like(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    Path((profile_id, post_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    if session.profile_id != profile_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    Ok(like::delete(&store.db_conn, profile_id, post_id).await?)
}
