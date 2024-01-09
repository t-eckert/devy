#![allow(dead_code, unused_imports)]
use opentelemetry::{global, trace::Tracer};
use opentelemetry_otlp::WithExportConfig;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use upload::{Git, Uploader};

mod api;
mod auth;
mod entities;
mod router;
mod store;
mod upload;

#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        Ok(_) => {
            tracing::debug!("Loaded .env file")
        }
        Err(_) => {
            tracing::warn!("Failed to load .env file");
        }
    }

    let tracer: opentelemetry_sdk::trace::Tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("api.honeycomb.io:443"),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .unwrap();

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(telemetry)
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    // Connect to the database.
    let db_connection_str = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!().run(&pool).await.unwrap();

    // Create the auth client
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not set");
    let post_auth_redirect_uri = env::var("POST_AUTH_URI").expect("POST_AUTH_URI not set");
    let callback_url = env::var("CALLBACK_URL").expect("CALLBACK_URL not set");
    let auth_client = auth::Client::new(
        client_id,
        client_secret,
        post_auth_redirect_uri,
        callback_url,
    );

    // Create the uploader.
    let git_path = env::var("GIT_PATH").expect("GIT_PATH not set");
    let git = Git::new(git_path).expect("Unable to create git client");
    let uploader = Uploader::new(git);

    let store = store::Store::new(pool, auth_client, uploader);

    let router = router::make_router(store);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
