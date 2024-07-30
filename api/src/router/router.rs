use super::middleware::auth;
use super::{endpoints, error::Result};
use axum::Extension;
use axum::{middleware, routing::get, Router as AxumRouter};
use lib::store::Store;
use lib::token::Session;
use std::net::SocketAddr;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

/// The main Router for the API.
pub struct Router {
    router: AxumRouter,
    socket_addr: SocketAddr,
}

impl Router {
    /// Create a new Router.
    pub fn new(store: Store, socket_addr: SocketAddr) -> Self {
        let router = axum::Router::new()
            .route("/ready", get(|| async { "OK" }))
            .route(
                "/protected",
                get(|Extension(session): Extension<Session>| async { axum::Json(session) }).layer(middleware::from_fn_with_state(store.clone(), auth)),
            )
            // Routers in Alphabetical Order
            .merge(endpoints::auth::router(store.clone()))
            .merge(endpoints::blogs::router(store.clone()))
            .merge(endpoints::feeds::router(store.clone()))
            .merge(endpoints::forms::router(store.clone()))
            .merge(endpoints::likes::router(store.clone()))
            .merge(endpoints::profiles::router(store.clone()))
            .merge(endpoints::uploads::router(store.clone()))
            .merge(endpoints::users::router(store.clone()))
            .merge(endpoints::webhooks::router(store.clone()))
            // Global middleware
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::new().allow_origin(Any))
            // State
            .with_state(store);

        Self {
            router,
            socket_addr,
        }
    }

    /// Start the server for the Router.
    pub async fn serve(self) -> Result<()> {
        let listener = tokio::net::TcpListener::bind(&self.socket_addr).await?;

        Ok(axum::serve(listener, self.router).await?)
    }
}
