use crate::{
    controllers::{AuthController, NewLogin},
    router::error::Result,
    store::Store,
};
use axum::{
    extract::State,
    response::Redirect,
    routing::{get, post},
    Json,
};
use lib::auth::Provider;
use std::collections::HashMap;

/// Create a new router for auth.
pub fn router() -> axum::Router<Store> {
    axum::Router::new()
        .route("/auth/providers", get(providers))
        .route("/auth/sign-in", post(sign_in))
}

// POST /auth/sign-in
async fn sign_in(State(store): State<Store>, Json(new_login): Json<NewLogin>) -> Redirect {
    match AuthController::sign_in(&store, &new_login).await {
        Ok(success_url) => Redirect::to(&success_url),
        Err(err) => Redirect::to(&format!(
            "{}?error={}",
            &new_login.error_url,
            err.to_string()
        )),
    }
}

// GET /auth/providers
async fn providers(State(store): State<Store>) -> Result<Json<HashMap<Provider, String>>> {
    Ok(Json(AuthController::providers(&store).await?))
}
