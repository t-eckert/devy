use auth::Client;
use db::connect;
use router::Router;
use std::net::SocketAddr;
use store::Store;

/// Start the API server.
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing::init();

    let auth_config = auth::Config::from_env().unwrap();
    let auth_client = Client::new(auth_config);

    let db_config = db::Config::from_env().unwrap();
    let db = connect(db_config).await.unwrap();

    let store = Store::new(db, auth_client);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let router = Router::new(store, addr);
    router.serve().await.unwrap();
}
