use crate::api::{
    auth, blogs, feed_configs, feeds, likes, posts, profiles, ready, repos, uploads, users,
    webhooks,
};
use crate::auth::is_authenticated;
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::store::Store;

pub fn make_router(store: Store) -> Router {
    // Allow CORS
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let open_routes = Router::new()
        .route("/auth/login", get(auth::login))
        .route("/auth/callback", get(auth::callback))
        .route("/blogs/:blog_slug", get(blogs::get_by_blog_slug))
        .route("/blogs/:blog_slug/posts", get(posts::get_by_blog_slug))
        .route(
            "/blogs/:blog_slug/posts/:post_slug",
            get(posts::get_by_blog_and_post_slug),
        )
        .route("/feeds/:id", get(feeds::get_feed_by_id))
        .route("/feeds/:id/posts", get(feeds::get_feed_posts_by_id))
        .route("/feeds/new/config", get(feed_configs::get_new))
        .route("/feeds/popular/config", get(feed_configs::get_popular))
        .route("/feeds/:id/config", get(feed_configs::get_by_id))
        .route("/posts/:post_id", get(posts::get_by_post_id))
        .route("/profiles/:username", get(profiles::get_by_username))
        .route("/profiles/:username/blogs", get(blogs::get_by_username))
        .route("/profiles/:username/posts", get(posts::get_by_username))
        .route(
            "/profiles/:username/likes",
            get(posts::get_liked_by_username),
        )
        .route(
            "/profiles/:username/likes/ids",
            get(likes::get_ids_by_username),
        )
        .route("/uploads/:username", get(uploads::get_by_username))
        .route("/users/:username", get(users::get_by_username))
        .route("/webhooks", post(webhooks::insert));

    let authed_routes = Router::new()
        .route("/blogs", post(blogs::upsert))
        .route("/blogs/:blog_slug", delete(blogs::delete))
        .route("/likes", post(likes::post_like))
        .route("/likes/:post_id/:profile_id", delete(likes::delete_like))
        .route("/repos", post(repos::insert))
        .route("/uploads", post(uploads::insert))
        .layer(middleware::from_fn(is_authenticated));

    let router = Router::new()
        .route("/v1/ready", get(ready::ready))
        .nest("/v1", open_routes)
        .nest("/v1", authed_routes)
        .with_state(store)
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer);

    return router;
}
