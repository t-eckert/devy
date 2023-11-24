use crate::api::{
    auth, blogs, feed_configs, feeds, likes, posts, profiles, ready, uploads, users, webhooks,
};
use axum::{
    routing::{delete, get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::store::Store;

pub fn make_router(store: Store) -> Router {
    // Allow CORS
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    let router = Router::new()
        // Auth
        .route("/v1/auth/login", get(auth::login))
        .route("/v1/auth/callback", get(auth::callback))
        // Blogs
        .route("/v1/blogs", post(blogs::upsert))
        .route("/v1/blogs/:blog_slug", get(blogs::get_by_blog_slug))
        .route("/v1/blogs/:blog_slug", delete(blogs::delete))
        .route("/v1/blogs/:blog_slug/posts", get(posts::get_by_blog_slug))
        .route(
            "/v1/blogs/:blog_slug/posts/:post_slug",
            get(posts::get_by_blog_and_post_slug),
        )
        // Feeds
        .route("/v1/feeds/:id", get(feeds::get_feed_by_id))
        .route("/v1/feeds/:id/posts", get(feeds::get_feed_posts_by_id))
        .route("/v1/feeds/:id/config", get(feed_configs::get_by_id))
        // Likes
        .route("/v1/likes", post(likes::post_like))
        .route("/v1/likes/:post_id/:profile_id", delete(likes::delete_like))
        // Posts
        .route("/v1/posts/:post_id", get(posts::get_by_post_id))
        // Profiles
        .route("/v1/profiles/:username", get(profiles::get_by_username))
        .route("/v1/profiles/:username/blogs", get(blogs::get_by_username))
        .route("/v1/profiles/:username/posts", get(posts::get_by_username))
        .route(
            "/v1/profiles/:username/likes",
            get(posts::get_liked_by_username),
        )
        .route(
            "/v1/profiles/:username/likes/ids",
            get(likes::get_ids_by_username),
        )
        // Ready
        .route("/v1/ready", get(ready::ready))
        // Uploads
        .route("/v1/uploads", post(uploads::insert))
        .route("/v1/uploads/:username", get(uploads::get_by_username))
        // Users
        .route("/v1/users/:username", get(users::get_by_username))
        // Webhooks
        .route("/v1/webhooks", post(webhooks::insert))
        .with_state(store)
        .layer(cors_layer);

    return router;
}
