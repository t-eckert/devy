use super::error::Result;
use crate::{entities::Upload, store::Store};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    Json,
};

pub async fn get_uploads_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Upload>>> {
    Ok(Json(Upload::get_all(&store.pool).await?))
}

pub async fn post_upload(
    State(store): State<Store>,
    ExtractJson(upload): ExtractJson<Upload>,
) -> Result<Json<Upload>> {
    Ok(Json(
        store
            .uploader
            .upload(upload.insert(&store.pool).await?, &store.pool)
            .await?,
    ))
}
