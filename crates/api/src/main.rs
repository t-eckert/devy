use auth::{Client, Providers};
use db::{connect, Config};
use router::Router;
use store::Store;

/// Start the API server.
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing::init();

    let auth_client = Client::new(Providers::GitHub);

    let config = Config::from_env().unwrap();
    let db = connect(config).await.unwrap();

    let store = Store::new(db, auth_client);

    let router = Router::new(store);
    router.serve().await;
}
