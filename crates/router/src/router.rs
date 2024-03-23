use crate::{error::Result, routers};
use axum::{routing::get, Router as AxumRouter};
use std::net::SocketAddr;
use store::Store;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

/// The main router for the API.
pub struct Router {
    router: AxumRouter,
    socket_addr: SocketAddr,
}

impl Router {
    /// Create a new router.
    pub fn new(store: Store, socket_addr: SocketAddr) -> Self {
        let v0_router = AxumRouter::new()
            .route("/ready", get(|| async { "OK" }))
            .merge(routers::BlogsRouter::create(store.clone()))
            .merge(routers::FeedsRouter::create(store.clone()))
            .merge(routers::AuthRouter::create(store.clone()))
            .merge(routers::ProfilesRouter::create(store.clone()))
            .merge(routers::FormsRouter::create(store.clone()))
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::new().allow_origin(Any))
            .with_state(store);

        let router = AxumRouter::new()
            .merge(v0_router.clone())
            .nest("/v1", v0_router);

        Self {
            router,
            socket_addr,
        }
    }

    /// Start the server.
    pub async fn serve(self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(&self.socket_addr).await?;

        Ok(axum::serve(listener, self.router).await?)
    }
}
