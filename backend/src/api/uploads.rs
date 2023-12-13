use super::error::Result;
use crate::{
    entities::{NewUpload, Upload, UploadRepository},
    store::Store,
};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    Json,
};

pub async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Upload>>> {
    println!("username: {}", username);
    Ok(Json(
        UploadRepository::get_by_username(&store.pool, &username).await?,
    ))
}

pub async fn insert(
    State(store): State<Store>,
    ExtractJson(upload): ExtractJson<NewUpload>,
) -> Result<Json<Upload>> {
    let received_upload = UploadRepository::insert(&store.pool, upload).await?;

    Ok(Json(
        store.uploader.upload(received_upload, &store.pool).await?,
    ))
}
