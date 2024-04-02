use auth::Client;
use db::connect;
use router::Router;
use std::net::SocketAddr;
use store::Store;

/// Start the API server.
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    monitoring::init();

    let auth_config = auth::Config::from_env().unwrap();
    let auth_client = Client::new(
        auth_config.client_id,
        auth_config.client_secret,
        auth_config.callback_url,
        auth_config.redirect_url,
    );

    let db_config = db::Config::from_env().unwrap();
    let db = connect(db_config).await.unwrap();

    let git_path = std::env::var("GIT_PATH").expect("GIT_PATH not set");
    let git = uploads::Git::new(git_path).expect("Unable to create git client");
    let uploader = uploads::Uploader::new(git);

    let store = Store::new(db, auth_client, uploader);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let router = Router::new(store, addr);
    router.serve().await.unwrap();
}
