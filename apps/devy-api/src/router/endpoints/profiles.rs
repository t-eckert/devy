use crate::{
    controllers::{BlogsController, EntriesController, ProfilesController},
    router::error::Result,
    store::Store,
};
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use lib::{blogs::Blog, posts::Entry, profile::Profile};

/// Create a new router for Profiles.
pub fn router(store: Store) -> Router<Store> {
    let open = Router::new()
        .route("/profiles/:username", get(get_profile_by_username))
        .route("/profiles/:username/blogs", get(get_blogs_by_username))
        .route("/profiles/:username/entries", get(get_entries_by_username))
        .with_state(store.clone());

    Router::new().merge(open)
}

// GET /profiles/:username
async fn get_profile_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    Ok(Json(
        ProfilesController::get_by_username(&store, username).await?,
    ))
}

// GET /profiles/:username/blogs
async fn get_blogs_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>> {
    Ok(Json(
        BlogsController::get_by_username(&store, &username).await?,
    ))
}

// GET /profiles/:username/entries
async fn get_entries_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Entry>>> {
    Ok(Json(
        EntriesController::get_by_username(&store, &username).await?,
    ))
}
