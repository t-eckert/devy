#![allow(dead_code)]
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod api;
mod auth;
mod entities;
mod forms;
mod router;
mod store;
mod upload;

#[tokio::main]
async fn main() {
    setup_env();
    setup_tracing();

    let pool = setup_db().await;
    let auth_client = auth::setup_client();
    let uploader = upload::setup_uploader();
    let store = store::Store::new(pool, auth_client, uploader);

    let router = router::make_router(store);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

fn setup_env() {
    match dotenvy::dotenv() {
        Ok(_) => {
            tracing::debug!("Loaded .env file")
        }
        Err(_) => {
            tracing::warn!("Failed to load .env file");
        }
    }
}

fn setup_tracing() {
    // Create a new OpenTelemetry trace pipeline that prints to stdout
    // let provider = TracerProvider::builder()
    //     .with_simple_exporter(
    //         opentelemetry_otlp::new_exporter()
    //             .tonic()
    //             .build_span_exporter()
    //             .unwrap(),
    //     )
    //     .build();

    // Create a tracing layer with the configured tracer
    // let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // // Use the tracing subscriber `Registry`, or any other subscriber
    // // that impls `LookupSpan`
    // let subscriber = Registry::default().with(telemetry);

    // // Trace executed code
    // tracing::subscriber::with_default(subscriber, || {
    //     // Spans will be sent to the configured OpenTelemetry exporter
    //     let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);
    //     let _enter = root.enter();

    //     error!("This event will be logged in the root span.");
    // });

    // Just make a program with just the start the span/end the span.

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

async fn setup_db() -> PgPool {
    let db_connection_str = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!().run(&pool).await.unwrap();

    pool
}
