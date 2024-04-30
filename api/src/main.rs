use lib::auth::Client;
use lib::db::connect;
use lib::monitoring;
use lib::router::Router;
use lib::store::Store;
use std::net::SocketAddr;

/// Start the API server.
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    monitoring::init();

    let auth_config = lib::auth::Config::from_env().unwrap();
    let auth_client = Client::new(
        auth_config.client_id,
        auth_config.client_secret,
        auth_config.callback_url,
        auth_config.redirect_url,
    );

    let db_config = lib::db::Config::from_env().unwrap();
    let db = connect(db_config).await.unwrap();

    let git_path = std::env::var("GIT_PATH").expect("GIT_PATH not set");
    let git = lib::uploader::Git::new(git_path).expect("Unable to create git client");
    let uploader = lib::uploader::Uploader::new(git);

    let store = Store::new(db, auth_client, uploader);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let router = Router::new(store, addr);
    router.serve().await.unwrap();
}
