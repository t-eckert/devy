use lib::{
    auth::Client, db::connect, github::GitHubClient, monitoring, router::Router, store::Store,
};
use std::net::SocketAddr;

mod config;

/// Start the API server.
#[tokio::main]
async fn main() {
    monitoring::init();

    let cfg = config::Config::from_env().unwrap();

    let auth_client = Client::new(
        cfg.github_app_client_id.clone(),
        cfg.github_app_client_secret,
        cfg.callback_url,
        cfg.redirect_url,
        cfg.encoding_key,
    );

    let db = connect(&cfg.database_url).await.unwrap();

    let git = lib::uploader::Git::new(&cfg.git_path).expect("Unable to create git client");
    let uploader = lib::uploader::Uploader::new(git);

    let github_client = GitHubClient::new(&cfg.github_app_client_id, &cfg.github_app_private_key);

    let store = Store::new(db, auth_client, uploader, github_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let router = Router::new(store, addr);
    router.serve().await.unwrap();
}
