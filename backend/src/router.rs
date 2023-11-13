use crate::api;
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
        .route("/v1/ready", get(api::ready::ready))
        .route("/v1/blogs", post(api::blogs::upsert_blog))
        .route(
            "/v1/blogs/:blog_slug",
            get(api::blogs::get_blog_by_blog_slug).delete(api::blogs::delete_blog),
        )
        .route(
            "/v1/blogs/:blog_slug/posts",
            get(api::blogs::get_blog_posts_by_blog_slug),
        )
        .route(
            "/v1/blogs/:blog_slug/posts/:post_slug",
            get(api::blogs::get_post_by_blog_and_post_slug),
        )
        .route("/v1/feeds/:id", get(api::feeds::get_feed_by_id))
        .route("/v1/feeds/:id/posts", get(api::feeds::get_feed_posts_by_id))
        .route("/v1/auth/login", get(api::auth::login))
        .route("/v1/auth/callback", get(api::auth::callback))
        .route("/v1/posts/:post_id", get(api::posts::get_post_by_post_id))
        .route(
            "/v1/profiles/:username",
            get(api::profiles::get_profile_by_username),
        )
        .route(
            "/v1/profiles/:username/blogs",
            get(api::profiles::get_blog_by_username),
        )
        .route(
            "/v1/profiles/:username/posts",
            get(api::profiles::get_posts_by_username),
        )
        .route(
            "/v1/profiles/:username/likes",
            get(api::profiles::get_liked_posts_by_username),
        )
        .route(
            "/v1/profiles/:username/likes/ids",
            get(api::profiles::get_likes_ids_by_username),
        )
        .route("/v1/uploads", post(api::uploads::post_upload))
        .route(
            "/v1/uploads/:username",
            get(api::uploads::get_uploads_by_username),
        )
        .route("/v1/users/:username", get(api::users::get_user_by_username))
        .route("/v1/likes", post(api::likes::post_like))
        .route(
            "/v1/likes/:post_id/:profile_id",
            delete(api::likes::delete_like),
        )
        .route("/v1/webhooks", post(api::webhooks::handle_webhook))
        .with_state(store)
        .layer(cors_layer);

    return router;
}
