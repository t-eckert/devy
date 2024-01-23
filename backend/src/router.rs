use crate::api::{
    auth, blogs, feeds, forms, likes, posts, profiles, repos, uploads, users, webhooks,
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
        .route("/blogs/:blog_slug", get(blogs::get_blog_by_blog_slug))
        .route("/blogs/:blog_slug/posts", get(posts::get_by_blog_slug)) // TODO move to blogs mod
        .route(
            "/blogs/:blog_slug/posts/:post_slug",
            get(blogs::get_post_by_blog_and_post_slug),
        )
        .route("/feeds/new/posts", get(feeds::get_posts_for_new))
        .route("/feeds/popular/posts", get(feeds::get_posts_for_popular))
        .route("/feeds/:id/posts", get(feeds::get_posts_by_feed_id))
        .route("/feeds/new/config", get(feeds::get_feed_config_for_new))
        .route(
            "/feeds/popular/config",
            get(feeds::get_feed_config_for_popular),
        )
        .route("/feeds/:id/config", get(feeds::get_feed_config_by_id))
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
        .route("/blogs/:blog_slug", delete(blogs::delete))
        .route("/forms/new-blog", post(forms::new_blog))
        .route("/likes", post(likes::post_like))
        .route("/likes/:post_id/:profile_id", delete(likes::delete_like))
        .route("/repos", post(repos::insert))
        .route("/uploads", post(uploads::insert))
        // .route("/uploads/:username", get(uploads::get_by_username))
        .layer(middleware::from_fn(is_authenticated));

    let router = Router::new()
        .route("/v1/ready", get(|| async { "OK" }))
        .nest("/v1", open_routes)
        .nest("/v1", authed_routes)
        .with_state(store)
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer);

    return router;
}
