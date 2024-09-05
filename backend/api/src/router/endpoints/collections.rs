use crate::{
    controllers::CollectionsController,
    router::{error::Result, middleware::auth},
    store::Store,
};
use axum::{extract::State, middleware, routing::get, Extension, Json};
use lib::{collection::Collection, token::Session};

/// Create a new router for Collections.
pub fn router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/collections/likes", get(get_likes))
        .route("/collections/bookmarks", get(get_bookmarks))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store)
}

// GET /collections/likes
async fn get_likes(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
) -> Result<Json<Collection>> {
    Ok(Json(
        CollectionsController::get_likes(&store, session.profile_id).await?,
    ))
}

// GET /collections/bookmarks
async fn get_bookmarks(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
) -> Result<Json<Collection>> {
    Ok(Json(
        CollectionsController::get_bookmarks(&store, session.profile_id).await?,
    ))
}
