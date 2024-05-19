use crate::{
    db::upload,
    entities::Upload,
    router::{error::Result, middleware::auth},
    store::Store,
};
use axum::{
    extract::{Json as ExtractJson, State},
    middleware,
    routing::post,
    Json,
};
use serde::{Deserialize, Serialize};

/// /uploads/*
///
///  Router for handling upload requests.
pub struct UploadsRouter;

impl UploadsRouter {
    /// Create a new UploadsRouter.
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/uploads", post(create_new_upload))
            .layer(middleware::from_fn_with_state(store.clone(), auth))
            .with_state(store)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewUpload {
    pub repo: String,
}

/// POST /uploads
///
/// Create a new upload.
async fn create_new_upload(
    State(store): State<Store>,
    ExtractJson(new_upload): ExtractJson<NewUpload>,
) -> Result<Json<Upload>> {
    Ok(Json(
        store
            .uploader
            .upload(
                &store.db,
                upload::insert(&store.db, None, new_upload.repo).await?,
            )
            .await?,
    ))
}
