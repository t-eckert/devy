use crate::auth::is_authenticated;
use axum::{middleware, routing::get, Router};

use crate::store::Store;

mod auth_router;
mod blogs_router;
mod error;
mod feeds_router;
mod forms_router;
mod likes_router;
mod profiles_router;
mod repos_router;
mod uploads_router;
mod users_router;
mod webhooks_router;

pub fn make_router(store: Store) -> Router {
    let router_v1 = Router::new()
        .route("/ready", get(|| async { "OK" }))
        .merge(auth_router::make_router(store.clone()))
        .merge(blogs_router::make_router(store.clone()))
        .merge(feeds_router::make_router(store.clone()))
        .merge(forms_router::make_router(store.clone()))
        .merge(likes_router::make_router(store.clone()))
        .merge(profiles_router::make_router(store.clone()))
        .merge(repos_router::make_router(store.clone()))
        .merge(uploads_router::make_router(store.clone()))
        .merge(users_router::make_router(store.clone()))
        .merge(webhooks_router::make_router(store.clone()))
        .layer(middleware::from_fn(is_authenticated));

    Router::new().nest("/v1", router_v1).with_state(store)
}
