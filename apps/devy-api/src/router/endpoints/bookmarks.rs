use crate::{
    controllers::{BookmarksController, NewBookmark},
    router::{error::Result, middleware::auth},
    store::Store,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use lib::{auth::token::Session, posts::Bookmark};
use uuid::Uuid;

/// Create a new router for bookmarks.
pub fn router(store: Store) -> Router<Store> {
    let open = Router::new()
        .route("/bookmarks/:username", get(get_by_username))
        .with_state(store.clone());

    let protected = Router::new()
        .route("/bookmarks", post(post_bookmark))
        .route("/bookmarks/:profile_id/:post_id", delete(delete_bookmark))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store);

    Router::new().merge(open).merge(protected)
}

// GET /bookmarks/:username
async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Bookmark>>> {
    Ok(Json(
        BookmarksController::get_by_username(&store, &username).await?,
    ))
}

// POST /bookmarks
async fn post_bookmark(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    Json(bookmark): Json<NewBookmark>,
) -> Result<Json<Bookmark>> {
    if session.profile_id != bookmark.profile_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    Ok(Json(BookmarksController::insert(&store, bookmark).await?))
}

// DELETE /bookmarks/:profile_id/:post_id
async fn delete_bookmark(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    Path((profile_id, post_id)): Path<(Uuid, Uuid)>,
) -> Result<()> {
    if session.profile_id != profile_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    BookmarksController::delete(&store, profile_id, post_id).await?;
    Ok(())
}
