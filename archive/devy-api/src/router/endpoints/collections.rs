use crate::{
    controllers::CollectionsController,
    router::{error::Result, middleware::auth},
    store::Store,
};
use axum::{
    extract::{Path, State},
    middleware,
    routing::get,
    Extension, Json,
};
use lib::{auth::token::Session, posts::Collection};

/// Create a new router for collections.
pub fn router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/collections/liked/:page", get(get_likes))
        .route("/collections/bookmarked/:page", get(get_bookmarks))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store)
}

// GET /collections/liked
async fn get_likes(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    Path(page): Path<u32>,
) -> Result<Json<Collection>> {
    Ok(Json(
        CollectionsController::get_likes(&store, session.profile_id, page).await?,
    ))
}

// GET /collections/bookmarked
async fn get_bookmarks(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    Path(page): Path<u32>,
) -> Result<Json<Collection>> {
    Ok(Json(
        CollectionsController::get_bookmarks(&store, session.profile_id, page).await?,
    ))
}
