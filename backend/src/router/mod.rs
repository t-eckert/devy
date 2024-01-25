use crate::api::{forms, likes, posts, profiles, repos, uploads, users, webhooks};
use crate::auth::is_authenticated;
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

use crate::store::Store;

mod auth_router;
mod blogs_router;
mod error;
mod feeds_router;
mod forms_router;
mod likes_router;
mod profiles_router;

pub fn make_router(store: Store) -> Router {
    let open_routes = Router::new()
        .route(
            "/profiles/:username",
            get(profiles::get_profile_by_username),
        )
        .route(
            "/profiles/:username/blogs",
            get(profiles::get_blogs_by_username),
        )
        .route(
            "/profiles/:username/posts",
            get(posts::get_posts_by_username),
        )
        .route(
            "/profiles/:username/likes",
            get(posts::get_liked_by_username),
        )
        .route(
            "/profiles/:username/likes/ids",
            get(likes::get_ids_by_username),
        )
        .route("/users/:username", get(users::get_by_username))
        .route("/webhooks", post(webhooks::insert));

    let authed_routes = Router::new()
        .route("/repos", post(repos::upsert))
        .route("/uploads", post(uploads::insert))
        // .route("/uploads/:username", get(uploads::get_by_username))
        .layer(middleware::from_fn(is_authenticated));

    let router_v1 = Router::new()
        .route("/ready", get(|| async { "OK" }))
        .merge(auth_router::make_router(store.clone()))
        .merge(blogs_router::make_router(store.clone()))
        .merge(feeds_router::make_router(store.clone()))
        .merge(forms_router::make_router(store.clone()))
        .merge(likes_router::make_router(store.clone()))
        .merge(open_routes)
        .merge(authed_routes);

    Router::new().nest("/v1", router_v1).with_state(store)
}
