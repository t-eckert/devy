use api::{feeds, ready};
use axum::{routing::get, Router};
use shuttle_secrets::SecretStore;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

mod api;
mod entities;

#[shuttle_runtime::main]
async fn axum(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let db_connection_str = secret_store
        .get("DB_CONN")
        .expect("DB_CONN environment variable not set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("Failed to connect to database");

    let router = Router::new()
        .route("/ready", get(ready))
        .nest("/feeds", feeds(pool));

    Ok(router.into())
}
