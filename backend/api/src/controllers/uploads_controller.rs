use super::Result;
use crate::store::Store;
use lib::uploader::Uploader;
use lib::uploads::{Upload, UploadRepository};
use serde::{Deserialize, Serialize};

pub struct UploadsController;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUpload {
    pub repo: String,
}

impl UploadsController {
    pub async fn get_uploads(store: &Store) -> Result<Vec<Upload>> {
        unimplemented!("get_uploads")
    }

    pub async fn create_new_upload(store: &Store, new_upload: NewUpload) -> Result<Upload> {
        unimplemented!("create_new_upload")
    }
}
