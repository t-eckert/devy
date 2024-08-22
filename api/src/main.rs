mod config;
mod router;

use lib::{auth, db, github, monitoring, store, uploader};
use std::net::SocketAddr;

/// Start the API server.
#[tokio::main]
async fn main() {
    monitoring::init();

    let cfg = config::Config::from_env().unwrap();

    let auth_client = auth::Client::new(
        cfg.github_app_client_id.clone(),
        cfg.github_app_client_secret,
        cfg.callback_url,
        cfg.redirect_url,
        cfg.encoding_private_key.clone(),
        cfg.encoding_private_key.as_bytes(),
        cfg.encoding_public_key.as_bytes(),
    );
    let db_conn = db::connect(&cfg.database_url).await.unwrap();
    let git = uploader::Git::new(&cfg.git_path).expect("Unable to create git client");
    let uploader = uploader::Uploader::new(git);
    let github_client = github::Client::new(&cfg.github_app_client_id, &cfg.github_app_private_key);

    let store = store::Store::new(db_conn, auth_client, uploader, github_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let router = router::Router::new(store, addr);
    router.serve().await.unwrap();
}
