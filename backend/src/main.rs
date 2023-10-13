use axum::{
    routing::{get, post},
    Router,
};
use shuttle_secrets::SecretStore;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod auth;
mod entities;
mod store;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Connect to the database.
    let db_connection_str = secret_store
        .get("DATABASE_URL")
        .expect("DATABASE_URL environment variable not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to database");

    // Create the auth client
    let client_id = secret_store
        .get("CLIENT_ID")
        .expect("CLIENT_ID environment variable not set");
    let client_secret = secret_store
        .get("CLIENT_SECRET")
        .expect("CLIENT_SECRET environment variable not set");
    let post_auth_redirect_uri = secret_store
        .get("POST_AUTH_REDIRECT_URI")
        .expect("POST_AUTH_REDIRECT_URI environment variable not set");
    let auth_client = auth::Client::new(client_id, client_secret, post_auth_redirect_uri);

    let store = store::Store::new(pool, auth_client);

    // Allow CORS
    let cors_layer = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods(Any)
        .allow_origin(Any);

    // Build the router.
    let router = Router::new()
        .route("/ready", get(api::ready::ready))
        .route("/blogs", post(api::blogs::upsert_blog))
        .route("/blogs/:blog_slug", get(api::blogs::get_blog_by_blog_slug))
        .route(
            "/blogs/:blog_slug/posts",
            get(api::blogs::get_blog_posts_by_blog_slug),
        )
        .route(
            "/blogs/:blog_slug/posts/:post_slug",
            get(api::blogs::get_post_by_blog_and_post_slug),
        )
        .route("/feeds/:id", get(api::feeds::get_feed_by_id))
        .route("/feeds/:id/posts", get(api::feeds::get_feed_posts_by_id))
        .route("/auth/login", get(api::auth::login))
        .route("/auth/callback", get(api::auth::callback))
        .route("/posts/:post_id", get(api::posts::get_post_by_post_id))
        .route("/posts/:post_id/likes", post(api::posts::post_like_to_post))
        .route(
            "/profiles/:username",
            get(api::profiles::get_profile_by_username),
        )
        .route(
            "/profiles/:username/blogs",
            get(api::profiles::get_blog_by_username),
        )
        .with_state(store)
        .layer(cors_layer);

    Ok(router.into())
}
