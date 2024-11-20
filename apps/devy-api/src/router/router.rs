use super::{endpoints, error::Result};
use axum::{routing::get, Router as AxumRouter};
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
    pub fn new(store: crate::store::Store, socket_addr: SocketAddr) -> Self {
        let router = axum::Router::new()
            .route("/ready", get(|| async { "OK" }))
            // Routers in Alphabetical Order
            .merge(endpoints::auth::router())
            .merge(endpoints::blogs::router(store.clone()))
            .merge(endpoints::bookmarks::router(store.clone()))
            .merge(endpoints::collections::router(store.clone()))
            .merge(endpoints::drafts::router(store.clone()))
            .merge(endpoints::feeds::router(store.clone()))
            .merge(endpoints::follows::router(store.clone()))
            .merge(endpoints::likes::router(store.clone()))
            .merge(endpoints::profiles::router(store.clone()))
            .merge(endpoints::search::router(store.clone()))
            .merge(endpoints::uploads::router(store.clone()))
            .merge(endpoints::users::router(store.clone()))
            .merge(endpoints::webhooks::router(store.clone()))
            // Global middleware
            .layer(TraceLayer::new_for_http())
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .allow_methods(vec![
                        http::Method::GET,
                        http::Method::POST,
                        http::Method::PUT,
                        http::Method::OPTIONS,
                        http::Method::DELETE,
                    ]),
            )
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
