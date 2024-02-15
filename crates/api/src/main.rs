use router::Router;

#[tokio::main]
async fn main() {
    let router = Router::new();
    router.serve().await;
}
