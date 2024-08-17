use crate::router::{error::Result, middleware::auth};
use axum::{
    extract::{Path, State},
    middleware,
    routing::get,
    Json, Router,
};
use lib::{
    db::{blog, entry, profile},
    entities::{Blog, Entry, Profile},
    store::Store,
};

/// `/profiles` endpoints
pub fn router(store: Store) -> Router<Store> {
    let open = Router::new()
        .route("/profiles/:username", get(get_profile_by_username))
        .route("/profiles/:username/blogs", get(get_blogs_by_username))
        .route("/profiles/:username/entries", get(get_entries_by_username))
        .route("/profiles/:username/following", get(get_following_by_username))
        .with_state(store.clone());

    let protected = Router::new()
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .route("/profiles", get(get_profiles))
        .with_state(store);

    Router::new().merge(open).merge(protected)
}

// GET /profiles
async fn get_profiles() -> Result<Json<Vec<Profile>>> {
    Ok(Json(vec![]))
}

// GET /profiles/:username
async fn get_profile_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    Ok(Json(profile::get_by_username(&store.db, username).await?))
}

// GET /profiles/:username/blogs
async fn get_blogs_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>> {
    Ok(Json(blog::get_by_username(&store.db, username).await?))
}

// GET /profiles/:username/entries
async fn get_entries_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Entry>>> {
    Ok(Json(entry::get_by_username(&store.db, &username).await?))
}

// GET /profiles/:username/following
async fn get_following_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>> {
    Ok(Json(vec![]))
}
