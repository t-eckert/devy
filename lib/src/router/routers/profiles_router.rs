use crate::db::{blog, entry, profile};
use crate::entities::{Blog, Entry, Profile};
use crate::router::error::Result;
use crate::store::Store;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

pub struct ProfilesRouter;

/// `/profiles`
impl ProfilesRouter {
    pub fn create(store: Store) -> Router<Store> {
        Router::new()
            .route("/profiles/:username", get(get_profile_by_username))
            .route("/profiles/:username/blogs", get(get_blogs_by_username))
            .route("/profiles/:username/entries", get(get_entries_by_username))
            .with_state(store)
    }
}

/// `GET /profiles/:username`
///
/// Get a profile by username.
async fn get_profile_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    Ok(Json(profile::get_by_username(&store.db, username).await?))
}

/// `GET /profiles/:username/blogs`
///
/// Get all blogs for a profile by username.
async fn get_blogs_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Blog>>> {
    Ok(Json(blog::get_by_username(&store.db, username).await?))
}

/// `GET /profiles/:username/entries`
async fn get_entries_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Entry>>> {
    Ok(Json(entry::get_by_username(&store.db, &username).await?))
}
