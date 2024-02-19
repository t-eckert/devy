use axum::{routing::get, Router as AxumRouter};
use std::net::SocketAddr;
use store::Store;

pub struct Router {
    axum_router: AxumRouter,
}

impl Router {
    pub fn new(store: Store) -> Self {
        let axum_router = AxumRouter::new()
            .route("/ready", get(|| async { "OK" }))
            .with_state(store);

        Self { axum_router }
    }

    pub async fn serve(self) {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, self.axum_router).await.unwrap();
    }
}
