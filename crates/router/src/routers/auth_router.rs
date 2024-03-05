use axum::{
    extract::{Query, State},
    response::Redirect,
    routing::get,
};
use std::collections::HashMap;
use store::Store;

/// /auth/*
///
/// Router for handling authentication requests.
pub struct AuthRouter;

impl AuthRouter {
    /// Create a new AuthRouter.
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/auth/login", get(login))
            .route("/auth/callback", get(callback))
            .with_state(store)
    }
}

/// GET /auth/login
///
/// Redirects the user to the login page.
async fn login(State(store): State<Store>) -> Redirect {
    Redirect::to(&store.auth_client.login())
}

/// GET /auth/callback
///
/// Handles the callback of the auth provider and redirect the user to the correct page.
async fn callback(
    State(store): State<Store>,
    Query(params): Query<HashMap<String, String>>,
) -> Redirect {
    Redirect::to(&store.auth_client.handle_callback(store.db, params).await)
}
