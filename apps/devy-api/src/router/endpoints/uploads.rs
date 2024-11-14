use crate::controllers::UploadsController;
use crate::router::{error::Result, middleware::auth};
use crate::store::Store;
use axum::{
    extract::{Json as ExtractJson, State},
    middleware,
    routing::{get, post},
    Json,
};
use http::StatusCode;
use lib::repositories::{BlogRepository, UploadRepository};
use lib::uploads::Upload;
use serde::{Deserialize, Serialize};

pub fn router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/uploads", get(get_uploads))
        .route("/uploads", post(create_new_upload))
        // .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store)
}

// GET /uploads
async fn get_uploads(State(store): State<Store>) -> Result<Json<Vec<Upload>>> {
    Ok(Json(UploadsController::get_uploads(&store).await?))
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
    dbg!(&new_upload);
    let blog = BlogRepository::get_by_repo(&store.db_conn, &new_upload.repo)
        .await?
        .ok_or(crate::router::error::Error::StatusCode(
            StatusCode::NOT_FOUND,
        ))?;
    let id = UploadRepository::insert(&store.db_conn, None, blog.id, &new_upload.repo).await?;
    let upload = UploadRepository::get(&store.db_conn, id).await?;
    Ok(Json(upload))
}
