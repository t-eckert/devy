use axum::{
    routing::{delete, get, post},
    Router,
};
use shuttle_secrets::SecretStore;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};

mod api;
mod auth;
mod entities;
mod router;
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

    let router = router::make_router(store);

    Ok(router.into())
}
