use axum::routing::{get, post};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = axum::Router::new()
        .route("/", post(|| async { "Received POST request" }))
        .route("/", get(|| async { "Received GET request" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000").await?;
    Ok(axum::serve(listener, router).await?)
}
