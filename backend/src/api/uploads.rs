use crate::{entities::Upload, store::Store};
use axum::{
    extract::{Json as ExtractJson, State},
    http::StatusCode,
    Json,
};

pub async fn get_uploads(State(store): State<Store>) -> Result<Json<Vec<Upload>>, StatusCode> {
    let uploads = match Upload::get_all(&store.pool).await {
        Ok(uploads) => uploads,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(uploads))
}

pub async fn post_upload(
    State(store): State<Store>,
    ExtractJson(payload): ExtractJson<Upload>,
) -> Result<Json<Upload>, StatusCode> {
    println!("Welcome to the uploads route!");

    let upload = match payload.insert(&store.pool).await {
        Ok(upload) => upload,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let received_upload = match store.uploader.upload(upload, &store.pool).await {
        Ok(upload) => upload,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    Ok(Json(received_upload))
}
