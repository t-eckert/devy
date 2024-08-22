use crate::router::{error::Result, middleware::auth};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    middleware,
    routing::{delete, get, post},
    Json, Router,
    Extension, http::StatusCode
};
use lib::{db::follow, store::Store, token::Session};
use uuid::Uuid;

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

async fn following(
    State(store): State<Store>,
    Path(profile_id): Path<Uuid>,
) -> Result<Json<Vec<follow::Follow>>> {
    Ok(Json(follow::get_by_profile_id(&store.db_conn, profile_id).await?))
}

async fn follow_blog(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    ExtractJson(follow): ExtractJson<follow::Follow>
) -> Result<()> {
    if session.profile_id != follow.profile_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    Ok(follow::insert(&store.db_conn, follow).await?)
}

async fn unfollow_blog(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    Path((profile_id, blog_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    if session.profile_id != profile_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    Ok(follow::delete(&store.db_conn, follow::Follow{profile_id, blog_id}).await?)
}
