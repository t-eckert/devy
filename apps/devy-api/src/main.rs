mod config;
mod controllers;
mod router;
mod store;

use lib::{auth, db, github, monitoring};
use std::net::SocketAddr;
use url::Url;

/// Start the API server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    monitoring::init();

    let cfg = config::Config::from_env()?;

    let encoder = auth::Encoder::new(
        cfg.encoding_private_key.as_bytes(),
        cfg.encoding_public_key.as_bytes(),
    )?;
    let github_provider = auth::providers::GitHubProvider::new(
        cfg.github_app_client_id.clone(),
        cfg.github_app_client_secret,
        Url::parse(&cfg.callback_url)?,
    )?;
    let auth_client = auth::Client::new(encoder, github_provider);
    let db_conn = db::connect(&cfg.database_url).await.unwrap();
    let github_client = github::Client::new(&cfg.github_app_client_id, &cfg.github_app_private_key);

    let store = store::Store::new(db_conn, auth_client, github_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let router = router::Router::new(store, addr);
    router.serve().await.unwrap();

    Ok(())
}
