use crate::router::{error::Result, middleware::auth};
use axum::{
    extract::{Json as ExtractJson, State},
    middleware,
    routing::post,
    Json,
};
use lib::{db::upload, entities::Upload };
use serde::{Deserialize, Serialize};
use crate::store::Store;

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
    Ok(Json(
        store
            .uploader
            .upload(
                &store.db_conn,
                upload::insert(&store.db_conn, None, new_upload.repo).await?,
            )
            .await?,
    ))
}
