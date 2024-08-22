use crate::router::{error::Result, middleware::auth};
use axum::{extract::State, routing::get,middleware, Extension,  Json};
use lib::{entities::Feed, store::Store, token::Session, controllers::FeedsController};

pub fn router(store: Store) -> axum::Router<Store> {
    let open =
        axum::Router::new()
            .route("/feeds/recent", get(get_recent))
            .route("/feeds/popular", get(get_popular))
            .with_state(store.clone());

    let protected =        axum::Router::new()
            .route("/feeds/following", get(get_following))
            .layer(middleware::from_fn_with_state(store.clone(), auth))
            .with_state(store);

    axum::Router::new().merge(open).merge(protected)
}

// GET /feeds/recent
async fn get_recent(State(store): State<Store>) -> Result<Json<Feed>> {
    Ok(Json(FeedsController::get_recent(&store).await?))
}

// GET /feeds/popular
async fn get_popular(State(store): State<Store>) -> Result<Json<Feed>> {
    Ok(Json(FeedsController::get_popular(&store).await?))
}

// Get /feeds/following
async fn get_following(
    Extension(session): Extension<Session>,
    State(store): State<Store>) -> Result<Json<Feed>> {
    Ok(Json(FeedsController::get_following(&store, session.profile_id).await?))
}
