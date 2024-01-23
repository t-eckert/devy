#![allow(dead_code)]
use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, trace::Tracer};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::TracerProvider;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tracing::{error, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use upload::{Git, Uploader};

mod api;
mod auth;
mod entities;
mod forms;
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

    // Create a new OpenTelemetry trace pipeline that prints to stdout
    let provider = TracerProvider::builder()
        .with_simple_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .build_span_exporter()
                .unwrap(),
        )
        .build();
    let tracer = provider.tracer("devy");
    let tracer_test = provider.tracer("test");

    let mut a = tracer_test.start("this");
    dbg!(&a.span_context());
    a.end();

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`, or any other subscriber
    // that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);

    // Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
        let _enter = root.enter();

        error!("This event will be logged in the root span.");
    });

    // Just make a program with just the start the span/end the span.

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .with(telemetry)
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
