use super::error::Result;
use crate::{
    entities::{upload, Upload},
    store::Store,
};
use axum::{
    extract::{Json as ExtractJson, Path, State},
    Json,
};

/// Get an upload given a user's username.
/// GET /uploads/:username
// pub async fn get_by_username(
//     State(store): State<Store>,
//     Path(username): Path<String>,
// ) -> Result<Json<Vec<Upload>>> {
//     println!("username: {}", username);
//     Ok(Json(
//         UploadRepository::get_by_username(&store.pool, &username).await?,
//     ))
// }

pub async fn insert(
    State(store): State<Store>,
    ExtractJson(upload): ExtractJson<upload::UploadForUpsert>,
) -> Result<Json<Upload>> {
    let received_upload = upload::insert(&store.pool, upload).await?;

    Ok(Json(
        store.uploader.upload(received_upload, &store.pool).await?,
    ))
}
