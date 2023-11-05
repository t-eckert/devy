use crate::entities::Upload;
use axum::{http::StatusCode, Json};

pub async fn post_upload() -> Result<Json<Upload>, StatusCode> {
    println!("Heyo!");
    Err(StatusCode::ACCEPTED)
}
