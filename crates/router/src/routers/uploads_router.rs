use super::error::Result;
use crate::entities::{upload, Upload};
use crate::store::Store;
use axum::{
    extract::{Json as ExtractJson, Path, State},
    Json,
};

pub fn make_router(store: Store) -> axum::Router<Store> {
    axum::Router::new()
        .route("/uploads", axum::routing::post(insert))
        .with_state(store)
}

/// GET /uploads/:username
///
/// Get an upload given a user's username.
async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Upload>>> {
    Ok(Json(upload::get_by_username(&store.pool, &username).await?))
}

/// POST /uploads
///
/// Create a new upload.
async fn insert(
    State(store): State<Store>,
    ExtractJson(upload): ExtractJson<upload::UploadForUpsert>,
) -> Result<Json<Upload>> {
    let received_upload = upload::insert(&store.pool, upload).await?;

    Ok(Json(
        store.uploader.upload(received_upload, &store.pool).await?,
    ))
}
