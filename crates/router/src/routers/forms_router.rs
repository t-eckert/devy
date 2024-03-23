use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post};
use store::Store;

pub struct FormsRouter;

impl FormsRouter {
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/forms/new-blog", post(new_blog))
            .with_state(store)
    }
}

/// GET /forms/new-blog
async fn new_blog(State(store): State<Store>) -> impl IntoResponse {
    (StatusCode::OK)
}
