use crate::{
    controllers::FollowsController,
    router::{error::Result, middleware::auth},
    store::Store,
};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use lib::{auth::token::Session, blogs::Follow};
use uuid::Uuid;

/// Create a new router for Follows.
pub fn router(store: Store) -> Router<Store> {
    let open = Router::new()
        .route("/follows/:profile_id", get(following))
        .with_state(store.clone());

    let protected = Router::new()
        .route("/follows", post(follow_blog))
        .route("/follows/:profile_id/:blog_id", delete(unfollow_blog))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store.clone());

    Router::new().merge(open).merge(protected)
}

// GET /follows/:profile_id
async fn following(
    State(store): State<Store>,
    Path(profile_id): Path<Uuid>,
) -> Result<Json<Vec<Follow>>> {
    Ok(Json(
        FollowsController::get_by_profile_id(&store, profile_id).await?,
    ))
}

// POST /follows
async fn follow_blog(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    ExtractJson(follow): ExtractJson<Follow>,
) -> Result<()> {
    if session.profile_id != follow.profile_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    Ok(FollowsController::insert(&store, follow).await?)
}

// DELETE /follows/:profile_id/:blog_id
async fn unfollow_blog(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    Path((profile_id, blog_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    if session.profile_id != profile_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    let follow = Follow {
        profile_id,
        blog_id,
    };

    Ok(FollowsController::delete(&store, follow).await?)
}
