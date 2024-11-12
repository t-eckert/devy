use crate::controllers::EntriesController;
use crate::router::{error::Result, middleware::auth};
use crate::store::Store;
use axum::{extract::State, middleware, routing::get, Extension, Json, Router};
use lib::{auth::token::Session, posts::Entry};

/// Create a new router for Drafts.
pub fn router(store: Store) -> Router<Store> {
    Router::new()
        .route("/drafts", get(get_drafts))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store)
}

// GET /drafts
async fn get_drafts(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
) -> Result<Json<Vec<Entry>>> {
    Ok(Json(
        EntriesController::get_drafts_by_profile_id(&store, session.profile_id).await?,
    ))
}
