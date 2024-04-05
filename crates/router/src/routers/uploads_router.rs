use crate::error::Result;
use axum::{
    extract::{Json as ExtractJson, Path, State},
    routing::{get, post},
    Json,
};
use db::upload;
use entities::Upload;
use store::Store;
use uploads::NewUpload;

/// /uploads/*
///
///  Router for handling upload requests.
pub struct UploadsRouter;

impl UploadsRouter {
    /// Create a new UploadsRouter.
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/uploads/:username", get(get_by_username))
            .route("/uploads", post(create_new_upload))
            .with_state(store)
    }
}

/// GET /uploads/:username
///
/// Get an upload given a user's username.
async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Vec<Upload>>> {
    Ok(Json(upload::get_by_username(&store.db, &username).await?))
}

/// POST /uploads
///
/// Create a new upload.
async fn create_new_upload(
    State(store): State<Store>,
    ExtractJson(new_upload): ExtractJson<NewUpload>,
) -> Result<Json<Upload>> {
    print!("\n\nVVVVV create_new_upload VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV\n");
    dbg!(&new_upload);

    Ok(Json(
        store
            .uploader
            .upload(
                &store.db,
                upload::insert(&store.db, None, new_upload.repo).await?,
            )
            .await?,
    ))
}
