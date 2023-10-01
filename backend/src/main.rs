use axum::{routing::get, Router};
use shuttle_secrets::SecretStore;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

mod api;
mod entities;

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

    // Build the router.
    let router = Router::new()
        .route("/ready", get(api::ready::ready))
        .route(
            "/blogs/:blog_slug/posts/:post_slug",
            get(api::blogs::get_post_by_blog_and_post_slug),
        )
        .route("/feeds/:id", get(api::feeds::get_feed_by_id))
        .route("/feeds/:id/posts", get(api::feeds::get_feed_posts_by_id))
        .with_state(pool);

    Ok(router.into())
}
