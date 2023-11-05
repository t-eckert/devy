use crate::{entities::Upload, store::Store};
use axum::{
    extract::{Json as ExtractJson, State},
    http::StatusCode,
    Json,
};

pub async fn post_upload(
    State(store): State<Store>,
    ExtractJson(payload): ExtractJson<Upload>,
) -> Result<Json<Upload>, StatusCode> {
    println!("Welcome to the uploads route!");

    let upload = match payload.insert(&store.pool).await {
        Ok(upload) => upload,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    /* x Insert the upload to the database.
     * Once inserted, pass the upload to the uploader.
     */

    Ok(Json(upload))
}
