use crate::router::{error::Result, middleware::auth};
use axum::{extract::State, routing::get,middleware, Extension,  Json};
use lib::{db::feed, entities::Feed, store::Store, token::Session};

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
    Ok(Json(feed::get_recent(&store.db).await?))
}

// GET /feeds/popular
async fn get_popular(State(store): State<Store>) -> Result<Json<Feed>> {
    Ok(Json(feed::get_popular(&store.db).await?))
}

// Get /feeds/following
async fn get_following(
    Extension(session): Extension<Session>,
    State(store): State<Store>) -> Result<Json<Feed>> {
    let profile_id = session.profile_id;
    Ok(Json(feed::get_following(&store.db, profile_id).await?))
}
