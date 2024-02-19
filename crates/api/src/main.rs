use auth::{Client, Providers};
use db::{connect, Config};
use router::Router;
use store::Store;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let auth_client = Client::new(Providers::GitHub);

    let config = Config::from_env().unwrap();
    let db = connect(config).await.unwrap();

    let store = Store::new(db, auth_client);

    let router = Router::new(store);
    router.serve().await;
}
