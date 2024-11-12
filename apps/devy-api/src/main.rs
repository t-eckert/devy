mod config;
mod controllers;
mod router;
mod store;

use lib::{auth, db, github, monitoring};
use std::net::SocketAddr;

/// Start the API server.
#[tokio::main]
async fn main() {
    monitoring::init();

    let cfg = match config::Config::from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Failed to load configuration: {:?}", err);
            match err {
                config::ConfigError::MissingEnv(e) => {
                    eprintln!("Missing environment variable: {}", e);
                }
            }
            std::process::exit(1);
        }
    };

    let auth_client = auth::Client::new(
        cfg.github_app_client_id.clone(),
        cfg.github_app_client_secret,
        cfg.callback_url,
        cfg.redirect_url,
        cfg.encoding_private_key.as_bytes(),
        cfg.encoding_public_key.as_bytes(),
    );
    let db_conn = db::connect(&cfg.database_url).await.unwrap();
    let github_client = github::Client::new(&cfg.github_app_client_id, &cfg.github_app_private_key);

    let store = store::Store::new(db_conn, auth_client, github_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let router = router::Router::new(store, addr);
    router.serve().await.unwrap();
}
