use crate::{error::Result, routers};
use axum::{routing::get, Router as AxumRouter};
use std::net::SocketAddr;
use store::Store;

/// The main router for the API.
pub struct Router {
    axum_router: AxumRouter,
    socket_addr: SocketAddr,
}

impl Router {
    /// Create a new router.
    pub fn new(store: Store, socket_addr: SocketAddr) -> Self {
        let axum_router = AxumRouter::new()
            .route("/ready", get(|| async { "OK" }))
            .merge(routers::BlogsRouter::create(store.clone()))
            .merge(routers::FeedsRouter::create(store.clone()))
            .with_state(store);

        Self {
            axum_router,
            socket_addr,
        }
    }

    /// Start the server.
    pub async fn serve(self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(&self.socket_addr).await?;

        Ok(axum::serve(listener, self.axum_router).await?)
    }
}
