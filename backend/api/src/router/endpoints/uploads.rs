use crate::router::{error::Result, middleware::auth};
use crate::store::Store;
use axum::{
    extract::{Json as ExtractJson, State},
    middleware,
    routing::post,
    Json,
};
use lib::uploader::Upload;
use serde::{Deserialize, Serialize};

pub fn router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/uploads", post(create_new_upload))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewUpload {
    pub repo: String,
}

// POST /uploads
async fn create_new_upload(
    State(store): State<Store>,
    ExtractJson(new_upload): ExtractJson<NewUpload>,
) -> Result<Json<Upload>> {
    let upload = Upload::new(&new_upload.repo, None);
    Ok(Json(store.uploader.upload(&store.db_conn, upload).await?))
}
